import "@moonbeam-network/api-augment";
import { beforeAll, describeSuite, expect, customDevRpcRequest } from "@moonwall/cli";

import { alith } from "@moonwall/util";
import { RELAY_SOURCE_LOCATION, relayAssetMetadata } from "../../../helpers/assets.js";
import { registerForeignAsset, XcmFragment } from "../../../helpers/xcm.js";
import type { XcmVersionedXcm } from "@polkadot/types/lookup";

// Twelve decimal places in the moonbase relay chain's token
const RELAY_TOKEN = 1_000_000_000_000n;

const palletId = "0x6D6f646c617373746d6E67720000000000000000";

describeSuite({
  id: "D3403",
  title: "Mock XCM V3 - downward transfer with always triggered appendix",
  foundationMethods: "dev",
  testCases: ({ context, it, log }) => {
    let assetId: string;

    beforeAll(async () => {
      // registerForeignAsset
      const { registeredAssetId, registeredAsset } = await registerForeignAsset(
        context,
        RELAY_SOURCE_LOCATION,
        relayAssetMetadata
      );
      assetId = registeredAssetId;
      expect(registeredAsset.owner.toHex()).to.eq(palletId.toLowerCase());
    });

    it({
      id: "T01",
      title: "Should make sure Alith receives 10 dot with appendix and without error",
      test: async function () {
        const xcmMessage = new XcmFragment({
          assets: [
            {
              multilocation: {
                parents: 1,
                interior: {
                  Here: null,
                },
              },
              fungible: 10n * RELAY_TOKEN,
            },
          ],
          beneficiary: alith.address,
        })
          .reserve_asset_deposited()
          .buy_execution()
          // Set an appendix to be executed after the XCM message is executed. No matter if errors
          .with(function () {
            return this.set_appendix_with([this.deposit_asset]);
          })
          .as_v2();

        const receivedMessage: XcmVersionedXcm = context
          .polkadotJs()
          .createType("StagingXcmVersionedXcm", xcmMessage);

        const totalMessage = [...receivedMessage.toU8a()];
        // Send RPC call to inject XCM message
        await customDevRpcRequest("xcm_injectDownwardMessage", [totalMessage]);

        // Create a block in which the XCM will be executed
        await context.createBlock();
        // Make sure the state has ALITH's to DOT tokens
        const alith_dot_balance = (
          await context.polkadotJs().query.assets.account(assetId, alith.address)
        )
          .unwrap()
          .balance.toBigInt();

        expect(alith_dot_balance, "Alith's DOT balance is empty").to.eq(10n * RELAY_TOKEN);
      },
    });
  },
});