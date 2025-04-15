export interface AnkiRpcTransport {
  request(serviceId: number, methodId: number, payload: any): Promise<any>;
}
