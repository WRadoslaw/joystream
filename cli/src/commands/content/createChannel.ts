import { getInputJson } from '../../helpers/InputOutput'
import { ChannelInputParameters } from '../../Types'
import { asValidatedMetadata, metadataToBytes } from '../../helpers/serialization'
import { flags } from '@oclif/command'
import { createType } from '@joystream/types'
import { ChannelCreationParameters } from '@joystream/types/content'
import { ChannelInputSchema } from '../../schemas/ContentDirectory'
import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import UploadCommandBase from '../../base/UploadCommandBase'
import chalk from 'chalk'
import { ChannelMetadata } from '@joystream/metadata-protobuf'

export default class CreateChannelCommand extends UploadCommandBase {
  static description = 'Create channel inside content directory.'
  static flags = {
    context: ContentDirectoryCommandBase.channelCreationContextFlag,
    input: flags.string({
      char: 'i',
      required: true,
      description: `Path to JSON file to use as input`,
    }),
  }

  async run(): Promise<void> {
    let { context, input } = this.parse(CreateChannelCommand).flags

    // Context
    if (!context) {
      context = await this.promptForChannelCreationContext()
    }
    const account = await this.getRequiredSelectedAccount()
    const actor = await this.getActor(context)
    const memberId = await this.getRequiredMemberId(true)
    await this.requestAccountDecoding(account)

    const channelInput = await getInputJson<ChannelInputParameters>(input, ChannelInputSchema)
    const meta = asValidatedMetadata(ChannelMetadata, channelInput)

    if (channelInput.collaborators) {
      await this.validateCollaborators(channelInput.collaborators)
    }

    const { coverPhotoPath, avatarPhotoPath } = channelInput
    const assetsPaths = [coverPhotoPath, avatarPhotoPath].filter((v) => v !== undefined) as string[]
    const resolvedAssets = await this.resolveAndValidateAssets(assetsPaths, input)
    // Set assets indexes in the metadata
    const [coverPhotoIndex, avatarPhotoIndex] = this.assetsIndexes([coverPhotoPath, avatarPhotoPath], assetsPaths)
    meta.coverPhoto = coverPhotoIndex
    meta.avatarPhoto = avatarPhotoIndex

    // Preare and send the extrinsic
    const assets = await this.prepareAssetsForExtrinsic(resolvedAssets)
    const channelCreationParameters = createType<ChannelCreationParameters, 'ChannelCreationParameters'>(
      'ChannelCreationParameters',
      {
        assets,
        meta: metadataToBytes(ChannelMetadata, meta),
        collaborators: channelInput.collaborators,
        reward_account: channelInput.rewardAccount,
      }
    )

    this.jsonPrettyPrint(JSON.stringify({ assets: assets?.toJSON(), metadata: meta }))

    await this.requireConfirmation('Do you confirm the provided input?', true)

    const result = await this.sendAndFollowNamedTx(account, 'content', 'createChannel', [
      actor,
      channelCreationParameters,
    ])

    const channelCreatedEvent = this.findEvent(result, 'content', 'ChannelCreated')
    const channelId = channelCreatedEvent!.data[1]
    this.log(chalk.green(`Channel with id ${chalk.cyanBright(channelId.toString())} successfully created!`))

    const dataObjectsUploadedEvent = this.findEvent(result, 'storage', 'DataObjectsUploaded')
    if (dataObjectsUploadedEvent) {
      const [objectIds] = dataObjectsUploadedEvent.data
      await this.uploadAssets(
        account,
        memberId,
        `dynamic:channel:${channelId.toString()}`,
        objectIds.map((id, index) => ({ dataObjectId: id, path: resolvedAssets[index].path })),
        input
      )
    }
  }
}
