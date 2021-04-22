import { SubstrateEvent } from '@dzlzv/hydra-common'
import { DatabaseManager } from '@dzlzv/hydra-db-utils'
import { u64 } from '@polkadot/types/primitive';

// Asset
import {
  DataObjectOwner,
  DataObject,
  LiaisonJudgement,
  Network,
} from 'query-node'
import {
  ContentParameters,
} from '@joystream/types/augment'

import { ContentParameters as Joystream_ContentParameters } from '@joystream/types/storage'
import { registry } from '@joystream/types'

const currentNetwork = Network.BABYLON

/*
  Reports that insurmountable inconsistent state has been encountered and throws an exception.
*/
export function inconsistentState(extraInfo: string, data?: unknown): void {
  const errorMessage = 'Inconsistent state: ' + extraInfo

  // log error
  logger.error(errorMessage, data)
}

/*
  Prepares data object from content parameters.
*/
export async function prepareDataObject(
  contentParameters: ContentParameters,
  blockNumber: number,
  owner: typeof DataObjectOwner,
): Promise<DataObject> {
  // convert generic content parameters coming from processor to custom Joystream data type
  const joystreamContentParameters = new Joystream_ContentParameters(registry, contentParameters.toJSON() as any)

  const dataObject = new DataObject({
    owner,
    createdInBlock: blockNumber,
    typeId: contentParameters.type_id.toNumber(),
    size: joystreamContentParameters.size_in_bytes.toNumber(),
    liaisonJudgement: LiaisonJudgement.PENDING, // judgement is pending at start; liaison id is set when content is accepted/rejected
    ipfsContentId: contentParameters.ipfs_content_id.toUtf8(),
    joystreamContentId: joystreamContentParameters.content_id.encode(),

    createdById: '1',
    updatedById: '1',
  })

  return dataObject
}

/////////////////// Logger /////////////////////////////////////////////////////

/*
  Simple logger enabling error and informational reporting.

  `Logger` class will not be needed in the future when Hydra v3 will be released.
  Hydra will provide logger instance and relevant code using `Logger` should be refactored.
*/
class Logger {

  /*
    Log significant event.
  */
  info(message: string, data?: unknown) {
    console.log(message, data)
  }

  /*
    Log significant error.
  */
  error(message: string, data?: unknown) {
    console.error(message, data)
  }
}

export const logger = new Logger()
