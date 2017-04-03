import Vue from 'vue';
import Router from 'vue-router';
import Controls from '@/components/Controls';

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Controls',
      component: Controls,
    },
  ],
});
