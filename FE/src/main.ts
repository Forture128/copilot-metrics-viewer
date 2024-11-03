import { createApp } from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import Toast from 'vue-toastification'
import './assets/globals.css' // shadcn-vue styles
// import "vue-toastification/dist/index.css"
import './assets/global.css' // your custom styles
import router from './router'
import store from './store'

// Toast options
const options = {
  // Your toast options
}

const app = createApp(App)

app.use(vuetify).use(store).use(Toast, options).use(router).mount('#app')
