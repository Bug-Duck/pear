import 'primeicons/primeicons.css'
import './assets/main.css'


import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import Tooltip from 'primevue/tooltip'

import router from './router'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'

import 'primeicons/primeicons.css'
import Ripple from 'primevue/ripple'

import { window } from '@tauri-apps/api'

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(PrimeVue, {
    theme: {
        preset: Aura
    },
    ripple: true
})

app.directive('tooltip', Tooltip)
app.directive('ripple', Ripple)

app.mount('#app')

requestIdleCallback(() => {
    window.getCurrentWindow().show()
})