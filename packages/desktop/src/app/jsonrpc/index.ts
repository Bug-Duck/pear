import { createInterface } from 'readline'
import { nanoid } from 'nanoid'
import { Readable, Writable } from 'stream'

export type MessageId = string | number

export interface BaseMessage {
  jsonrpc: '2.0'
  id?: MessageId
}

export interface InvokeMessage<TParams = never> extends Required<BaseMessage> {
  method: string
  params?: TParams
}

export interface InvokeResponseOk<T> extends Required<BaseMessage> {
  result: T
}

export interface InvokeResponseError<E = unknown>
  extends Required<BaseMessage> {
  error: InvokeError<E>
}

export interface InvokeError<E = unknown> {
  code: number
  message: string
  data?: E
}

export type InvokeResponse<T, E = unknown> =
  | InvokeResponseOk<T>
  | InvokeResponseError<E>

export type NotificationMessage<TParams = never> = Omit<
  InvokeMessage<TParams>,
  'id'
>

export type Socket = [
  receiver: (onRecv: (data: Uint8Array | string) => void) => void,
  sender: (data: Uint8Array) => void,
]

export type Deferred<T> = Promise<T> & {
  resolve(value: T): void
  reject(error: any): void
}

export type Handler<T extends NotificationMessage<unknown>> =
  T extends NotificationMessage<infer P> ? (params: P) => void : () => void

export type ClientState = {
  callbacks: Map<MessageId, Deferred<unknown>>
  handlers: Map<string, Array<(params: unknown) => void>>
}

export type Client = {
  state: ClientState
  call: <T>(method: string, params: unknown) => Promise<T>
  batch: <T extends [unknown] | unknown[]>(
    call: [method: string, params: unknown][]
  ) => Promise<T>
}

export const deferred = <T>(): Deferred<T> => {
  let resolveFn, rejectFn
  const promise = new Promise<T>((resolve, reject) => {
    resolveFn = resolve
    rejectFn = reject
  }) as Deferred<T>

  promise.resolve = resolveFn!
  promise.reject = rejectFn!

  return promise
}

export const useClient = (
  [onRecv, send]: Socket,
  errorHandler?: (err: any) => void
): Client => {
  const encoder = new TextEncoder(),
    decoder = new TextDecoder()
  const state: ClientState = { callbacks: new Map(), handlers: new Map() }

  const handleMessage = (obj: any) => {
    if (typeof obj.id == 'string' || typeof obj.id == 'number') {
      const callback = state.callbacks.get(obj.id)
      if (!callback) return

      state.callbacks.delete(obj.id)
      if (obj.error != null) {
        callback.reject(
          new Error(
            `RPC Error (code ${obj.error.code}): ${obj.error.message}`,
            { cause: obj }
          )
        )
      } else if (obj.result != null) {
        callback.resolve(obj.result)
      }
    } else if (typeof obj.method == 'string') {
      state.handlers.get(obj.method)?.forEach((f) => f(obj.params))
    } else if (Array.isArray(obj)) {
      obj.forEach((o) => handleMessage(o))
    }
  }

  onRecv((data) => {
    try {
      const text =
        typeof data == 'string' ? data.trim() : decoder.decode(data).trim()
      const obj = JSON.parse(text)
      handleMessage(obj)
    } catch (err: any) {
      console.error('RPC client error when recv:', err)
      errorHandler?.(err)
    }
  })

  return {
    state,
    call: <T, P = never>(method: string, params: P) => {
      const d = deferred<T>()
      const id = nanoid()

      const message = {
        jsonrpc: '2.0',
        id,
        method,
        params,
      } satisfies InvokeMessage<unknown>

      const data = encoder.encode(JSON.stringify(message))

      state.callbacks.set(id, d)
      send(data)

      return d
    },

    batch: <T extends [unknown] | unknown[]>(
      call: [method: string, params: unknown][]
    ): Promise<T> => {
      const message = call.map(([method, params]) => ({
        jsonrpc: '2.0',
        id: nanoid(),
        method,
        params,
      }))

      const data = encoder.encode(JSON.stringify(message))

      const d = message.map(({ id }) => {
        const d = deferred<unknown>()
        state.callbacks.set(id, d)
        return d
      })

      send(data)

      return Promise.all(d) as any
    },
  } satisfies Client
}

export const toSocket = (subout: Readable, subin: Writable): Socket => {
  const rl = createInterface({
    input: subout,
    terminal: false,
  })

  return [
    (callback) => {
      rl.on('line', (input) => callback(input))
    },
    (data) => {
      subin.write(Buffer.from(data))
      subin.write('\r\n')
    },
  ]
}
