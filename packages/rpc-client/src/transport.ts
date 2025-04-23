import {
  Deferred,
  RpcTransport,
  UnaryCall,
  MethodInfo,
  RpcMetadata,
  RpcStatus,
  RpcOptions,
  DuplexStreamingCall,
  ClientStreamingCall,
  ServerStreamingCall,
} from '@protobuf-ts/runtime-rpc';
import { SERVICES, METHODS, ServiceMapType } from './generated/service.ts';
import { BackendError } from './generated/protobuf/anki/backend.ts';
import { toCamelCase } from 'remeda';

export interface AnkiRpcTransport
  extends Partial<Pick<RpcTransport, 'mergeOptions'>> {
  request(
    serviceId: number,
    methodId: number,
    payload: Uint8Array
  ): Promise<[Uint8Array, undefined] | [undefined, Uint8Array]>;
}

const notImplemented = (): never => {
  throw new Error('not implemented');
};

export class Transport implements RpcTransport {
  constructor(private transport: AnkiRpcTransport) {}

  mergeOptions(options?: Partial<RpcOptions>): RpcOptions {
    return this.transport.mergeOptions?.(options) || {};
  }

  unary<I extends object, O extends object>(
    method: MethodInfo<I, O>,
    input: I,
    options: RpcOptions
  ): UnaryCall<I, O> {
    const serviceName = toCamelCase(
      method.service.typeName
        .split('.')
        .pop()
        ?.replace(/^Backend/, '')
        ?.replace(/Service$/, '') as keyof ServiceMapType
    );
    const serviceMeta = SERVICES[serviceName];
    if (!serviceMeta) {
      throw new Error('service not found');
    }
    const serviceId = serviceMeta[0];

    const methodId = (METHODS[serviceName] as never)[method.localName];

    const defHeader = new Deferred<RpcMetadata>(),
      defMessage = new Deferred<O>(),
      defStatus = new Deferred<RpcStatus>(),
      defTrailer = new Deferred<RpcMetadata>();

    defHeader.resolve({});
    this.transport
      .request(serviceId, methodId, method.I.toBinary(input))
      .then((res) => {
        const [value, err] = res;
        if (err) {
          defMessage.reject(BackendError.fromBinary(err));
        } else {
          defMessage.resolve(method.O.fromBinary(value));
        }
        defTrailer.resolve({});
        defStatus.resolve({
          code: 'OK',
          detail: 'OK',
        });
      });

    return new UnaryCall(
      method,
      options.meta || {},
      input,
      defHeader.promise,
      defMessage.promise,
      defStatus.promise,
      defTrailer.promise
    );
  }

  duplex<I extends object, O extends object>(): DuplexStreamingCall<I, O> {
    return notImplemented();
  }

  clientStreaming<I extends object, O extends object>(): ClientStreamingCall<
    I,
    O
  > {
    return notImplemented();
  }

  serverStreaming<I extends object, O extends object>(): ServerStreamingCall<
    I,
    O
  > {
    return notImplemented();
  }
}
