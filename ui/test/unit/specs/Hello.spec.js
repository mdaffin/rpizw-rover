import Vue from 'vue';
import Controls from '@/components/Controls';

describe('Controls.vue', () => {
  it('should render correct contents', () => {
    const Constructor = Vue.extend(Controls);
    const vm = new Constructor().$mount();
    expect(vm.$el.querySelector('.hello h1').textContent)
      .to.equal('Welcome to Your Vue.js App');
  });
});
