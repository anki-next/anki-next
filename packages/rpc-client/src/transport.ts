import { RpcTransport } from '@protobuf-ts/runtime-rpc';

export interface AnkiRpcTransport extends Pick<RpcTransport, 'mergeOptions'> {
  request(
    serviceId: number,
    methodId: number,
    payload: Uint8Array
  ): Promise<Uint8Array>;
}

export function createRpcTransport(transport: AnkiRpcTransport): RpcTransport {
  const notImplemented = () => {
    throw new Error('not implemented');
  };
  return {
    mergeOptions(options) {
      return transport.mergeOptions(options);
    },
    unary(_method, _input, _options) {
      throw new Error('todo');
    },
    serverStreaming: notImplemented,
    clientStreaming: notImplemented,
    duplex: notImplemented,
  };
}
