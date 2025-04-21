import { Button } from '@anki-next/ui/components/button';
import { AnkiRpcTransport, createAnkiService } from '@anki-next/rpc-client';

const service = createAnkiService({
  async request(serviceId, methodId, payload) {
    const response = await fetch(
      `http://localhost:3001/rpc/${serviceId}/${methodId}`,
      {
        method: 'POST',
        body: payload,
      }
    );
    if (response.ok) {
      const hasError = response.headers.get('x-rpc-error');
      const body = await response.bytes();
      return hasError ? [undefined, body] : [body, undefined];
    }
    throw new Error('fetch error');
  },
});

(window as any).service = service;

(async function () {
  console.log(await service.collection.openCollection({}));
})();

export function App() {
  return (
    <div className="flex items-center justify-center min-h-svh">
      <div className="flex flex-col items-center justify-center gap-4">
        <h1 className="text-2xl font-bold">Hello World</h1>
        <Button size="sm">Button</Button>
      </div>
    </div>
  );
}

export default App;
