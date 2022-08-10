import { FlowProps } from '../../Flow'
import { SudoUpdateMembershipSystem } from '../../fixtures/membership'

import { extendDebug } from '../../Debugger'
import { FixtureRunner } from '../../Fixture'
import BN from 'bn.js'

export default async function membershipSystem({ api, query, env }: FlowProps): Promise<void> {
  const debug = extendDebug('flow:membership-system')
  debug('Started')
  api.enableDebugTxLogs()

  // testing updates, keeping final membership price significantly larger
  // than buy membership tx fee, or BuyMembershipWithInsufficienFundsFixture would fail.
  const updates = [
    // BigInt above Int32 case:
    {
      defaultInviteCount: 10,
      membershipPrice: new BN(100_000_000_000),
      referralCut: 5,
      invitedInitialBalance: new BN(3_000_000),
    },
    {
      defaultInviteCount: 5,
      membershipPrice: new BN(100_000_000),
    },
    {
      referralCut: 0,
      invitedInitialBalance: new BN(5_000_00),
    },
  ]

  const fixtures = updates.map((u) => new SudoUpdateMembershipSystem(api, query, u))
  const runners = fixtures.map((f) => new FixtureRunner(f))
  // Fixtures should be executed one-by-one to not interfere with each other (before->after snapshot checks)
  for (const key in runners) {
    debug(`Running update fixture number ${parseInt(key) + 1}`)
    await runners[key].run()
  }

  debug('Running query node checks')
  await Promise.all(runners.map((r) => r.runQueryNodeChecks()))

  debug('Done')
}
