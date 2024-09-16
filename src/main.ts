import { createApp } from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import { loadFonts } from './plugins/webfontloader'
import Toast from 'vue-toastification';
import './assets/global.css';
// Import the CSS or use your own!
import "vue-toastification/dist/index.css";

loadFonts()
const options = {
  // You can set your default options here
};


createApp(App)
  .use(vuetify)
  .use(Toast, options)
  .mount('#app')
