import test from 'ava'

import { LycheeServer, plus100 } from '../index'

test('sync function from native code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
})

test('run WS', async (t) => {
  const server = new LycheeServer()
  await server.startServer()
  server.doSomethingElse()
  t.assert(true)
})
