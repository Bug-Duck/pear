import { $, os } from 'zx'

$.verbose = true

if (os.platform() == 'win32') {
  // Windows compatibility
  $.shell = 'powershell'
  $.prefix = ''
}

export { default as throttle }  from 'lodash.throttle'

export * from 'zx'
