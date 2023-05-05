import "@moonbeam-network/api-augment";
import { ApiDecoration } from "@polkadot/api/types";
import chalk from "chalk";
import { expect, beforeAll, describeSuite } from "@moonwall/cli";
import type { PalletProxyProxyDefinition } from "@polkadot/types/lookup";

describeSuite({
  id: "S1600",
  title: "Verify account proxies created",
  foundationMethods: "read_only",
  testCases: ({ context, it, log }) => {
    const proxiesPerAccount: { [account: string]: PalletProxyProxyDefinition[] } = {};
    const proxyAccList = [];

    let atBlockNumber: number = 0;
    let apiAt: ApiDecoration<"promise"> = null;
    let specVersion: number = 0;

    beforeAll(async function () {
      const limit = 1000;
      let last_key = "";
      let count = 0;

      // Configure the api at a specific block
      // (to avoid inconsistency querying over multiple block when the test takes a long time to
      // query data and blocks are being produced)
      atBlockNumber = process.env.BLOCK_NUMBER
        ? parseInt(process.env.BLOCK_NUMBER)
        : (await context.polkadotJs().rpc.chain.getHeader()).number.toNumber();
      apiAt = await context.polkadotJs().at(
        await context.polkadotJs().rpc.chain.getBlockHash(atBlockNumber)
      );
      specVersion = (await apiAt.query.system.lastRuntimeUpgrade()).unwrap().specVersion.toNumber();

      // TEMPLATE: query the data
      while (true) {
        let query = await apiAt.query.proxy.proxies.entriesPaged({
          args: [],
          pageSize: limit,
          startKey: last_key,
        });

        if (query.length == 0) {
          break;
        }
        count += query.length;

        // TEMPLATE: convert the data into the format you want (usually a dictionary per account)
        for (const proxyData of query) {
          let accountId = `0x${proxyData[0].toHex().slice(-40)}`;
          last_key = proxyData[0].toString();
          proxiesPerAccount[accountId] = proxyData[1][0].toArray();
          proxyAccList.push(accountId);
        }

        // log logs to make sure it keeps progressing
        // TEMPLATE: Adapt log line
        if (count % (10 * limit) == 0) {
          log(`Retrieved ${count} proxies`);
        }
      }

      // TEMPLATE: Adapt proxies
      log(`Retrieved ${count} total proxies`);
    }, 30_000);

    // TEMPLATE: Give details about what you are testing
    it({id:"C100",title: "should have no more than the maximum allowed proxies",timeout: 240000, test: async function () {
      const maxProxies = (await context.polkadotJs().consts.proxy.maxProxies).toNumber();
      const failedProxies: { accountId: string; proxiesCount: number }[] = [];

      for (const accountId of Object.keys(proxiesPerAccount)) {
        const proxiesCount = proxiesPerAccount[accountId].length;
        if (proxiesCount > maxProxies) {
          failedProxies.push({ accountId, proxiesCount });
        }
      }

      if (failedProxies.length > 0) {
        log("Failed accounts with too many proxies:");
        log(
          failedProxies
            .map(({ accountId, proxiesCount }) => {
              return `accountId: ${accountId} - ${chalk.red(
                proxiesCount.toString().padStart(4, " ")
              )} proxies (expected max: ${maxProxies})`;
            })
            .join(`\n`)
        );
      }

      expect(failedProxies.length, "Failed max proxies").to.equal(0);

      log(
        `Verified ${Object.keys(proxiesPerAccount).length} total accounts (at #${atBlockNumber})`
      );
    }});

    it({id:"C200",title: "should have a maximum allowed proxies of 32",test: async function () {
      const runtimeName = context.polkadotJs().runtimeVersion.specName.toString();
      const networkName = context.polkadotJs().runtimeChain.toString();
      const maxProxies = (await context.polkadotJs().consts.proxy.maxProxies).toNumber();

      switch (runtimeName) {
        case "moonbase":
          expect(maxProxies).to.equal(32);
          break;
        case "moonriver":
          expect(maxProxies).to.equal(32);
          break;
        case "moonbeam":
          expect(maxProxies).to.equal(32);
          break;
        default:
          expect(maxProxies).to.equal(32);
          break;
      }

      // TEMPLATE: This is redundant but is used to show how to check based on the network
      switch (networkName) {
        case "Moonbase Alpha":
          expect(maxProxies).to.equal(32);
          break;
        case "Moonbeam":
          expect(maxProxies).to.equal(32);
          break;
        default:
          expect(maxProxies).to.equal(32);
          break;
      }

      // TEMPLATE: Updates the log line
      log(`Verified maximum allowed proxies constant`);
    }});

    it({
      id:"C300",
      title:"should only be possible for proxies of non-smart contract accounts",
      timeout:60000,
      test:async function () {
        // For each account with a registered proxy, check whether it is a non-SC address
        const results = await Promise.all(
          proxyAccList.map(async (address) => {
            const resp = await apiAt.query.evm.accountCodes(address);
            const contract = resp.toJSON() == "0x" ? false : true;
            return { address, contract };
          })
        );
        results.forEach((item) => {
          if (item.contract)
            log(`Proxy account for non-external address detected: ${item.address} `);
        });
        expect(results.every((item) => item.contract == false)).to.be.true;
      }
    });
  },
});
