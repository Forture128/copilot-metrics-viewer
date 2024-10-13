import { createApp } from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import { loadFonts } from './plugins/webfontloader'
import Toast from 'vue-toastification';
import './assets/global.css';
// Import the CSS or use your own!
import "vue-toastification/dist/index.css";
import router from './router';
import store from './store/DoraData';
loadFonts()
const options = {
  // You can set your default options here
};


createApp(App)
  .use(vuetify)
  .use(store)
  .use(Toast, options)
  .use(router)
  .mount('#app')
