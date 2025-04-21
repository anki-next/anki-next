/* eslint-disable @typescript-eslint/no-explicit-any */
import { RpcTransport } from '@protobuf-ts/runtime-rpc';
import { AnkiService, METHODS, SERVICES } from './generated/service.ts';
import { AnkiRpcTransport, Transport } from './transport';

interface ClientConstructor {
  new (transport: RpcTransport): any;
}

export function createAnkiService(transport: AnkiRpcTransport): AnkiService {
  const rpcTransport = new Transport(transport);
  const clients = new Map<ClientConstructor, object>();

  function clientInstance(ctor: ClientConstructor): any {
    if (clients.has(ctor)) {
      return clients.get(ctor);
    }
    const instance = new ctor(rpcTransport);
    clients.set(ctor, instance);
    return instance;
  }

  return Object.fromEntries(
    Object.entries(SERVICES).map(([serviceName, serviceMeta]) => {
      const [, backendServiceCtor, serviceCtor] = serviceMeta;
      return [
        serviceName,
        Object.fromEntries(
          Object.keys(METHODS[serviceName as keyof AnkiService]).map(
            (methodName) => {
              return [
                methodName,
                (...args: any[]) => {
                  for (const ctor of [backendServiceCtor, serviceCtor]) {
                    const instance = clientInstance(ctor);
                    if (methodName in instance) {
                      return instance[methodName].apply(instance, args);
                    }
                  }
                },
              ];
            }
          )
        ) as any,
      ];
    })
  ) as AnkiService;
}
