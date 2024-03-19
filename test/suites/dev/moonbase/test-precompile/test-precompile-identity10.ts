import "@moonbeam-network/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { alith, charleth } from "@moonwall/util";
import { expectEVMResult, PRECOMPILE_IDENTITY_ADDRESS } from "../../../../helpers";

describeSuite({
  id: "D012944",
  title: "Precompiles - Identity precompile - set account id",
  foundationMethods: "dev",
  testCases: ({ it, log, context }) => {
    beforeAll(async function () {
      await context.createBlock(
        context
          .polkadotJs()
          .tx.sudo.sudo(context.polkadotJs().tx.identity.addRegistrar(alith.address))
      );

      const block = await context.createBlock(
        await context.writeContract!({
          contractName: "Identity",
          contractAddress: PRECOMPILE_IDENTITY_ADDRESS,
          functionName: "setAccountId",
          rawTxOnly: true,
          args: [0, charleth.address],
        })
      );

      expectEVMResult(block.result!.events, "Succeed");
    });

    it({
      id: "T01",
      title: "should retrieve the registrar",
      test: async function () {
        const registrars = (await context.readContract!({
          contractAddress: PRECOMPILE_IDENTITY_ADDRESS,
          contractName: "Identity",
          functionName: "registrars",
        })) as any;

        expect(registrars.length).to.equal(1);
        expect(registrars[0].isValid).to.be.true;
        expect(registrars[0].index).to.equal(0);
        expect(registrars[0].account).to.equal(charleth.address);
        expect(registrars[0].fee).to.equal(0n);
        expect(registrars[0].fields.display).to.be.false;
        expect(registrars[0].fields.web).to.be.false;
        expect(registrars[0].fields.legal).to.be.false;
        expect(registrars[0].fields.riot).to.be.false;
        expect(registrars[0].fields.email).to.be.false;
        expect(registrars[0].fields.pgpFingerprint).to.be.false;
        expect(registrars[0].fields.image).to.be.false;
        expect(registrars[0].fields.twitter).to.be.false;
      },
    });
  },
});
