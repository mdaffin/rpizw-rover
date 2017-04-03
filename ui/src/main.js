// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from 'vue';
import VueResource from 'vue-resource';
import App from './App';
import router from './router';

import '../node_modules/concise.css/dist/concise.css';
import '../node_modules/concise-ui/dist/concise-ui.css';

Vue.config.productionTip = false;
Vue.use(VueResource);
Vue.http.options.xhr = { withCredentials: true };

/* eslint-disable no-new */
new Vue({
  el: '#app',
  router,
  template: '<App/>',
  components: { App },
});
