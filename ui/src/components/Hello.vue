<template>
  <main>
    <section class="alert-box -error"
             v-if="errorMessage !== false">
      <p>{{ errorMessage }}</p>
      <a class="close"
         href="#">×</a>
    </section>
    <section>
      <div class="controls">
        <form>
          <div grid>
            <div column>
              <label>
                <span>Left</span>
                <input type="range"
                       id="left"
                       v-model="left"
                       max="10"
                       min="-10"
                       value="0">
              </label>
            </div>
            <div column>
              <label>
                <span>Right</span>
                <input type="range"
                       id="right"
                       v-model="right"
                       max="10"
                       min="-10"
                       value="0">
              </label>
            </div>
          </div>
          <div>
            <button @click="stop">Stop</button>
            <button @click="toggleEnabled">
              <span v-if="enabled">Disable</span>
              <span v-else>Enable</span>
            </button>
            <button @click="reset">Reset</button>
          </div>
        </form>
      </div>
    </section>
  </main>
</template>

<script>
// import nipplejs from 'nipplejs';

// nipplejs.create({
//   color: 'blue',
// });

const API_URL = 'http://rpizw-rover.local:3000';

export default {
  name: 'hello',
  data() {
    return {
      left: '10',
      right: '10',
      errorMessage: false,
      locked: true,
      enabled: true,
      wPressed: false,
      aPressed: false,
      sPressed: false,
      dPressed: false,
    };
  },
  created() {
    window.addEventListener('keyup', this.keyReleased);
    window.addEventListener('keydown', this.keyPressed);
  },
  methods: {
    toggleEnabled() {
      this.$set(this, 'enabled', !this.enabled);
      if (this.enabled) {
        this.$http.put(`${API_URL}/api/enable`).then(() => { }, (response) => {
          this.errorMessage = response;
        });
      } else {
        this.$http.put(`${API_URL}/api/disable`).then(() => { }, (response) => {
          this.errorMessage = response;
        });
      }
    },
    stop() {
      this.left = 0;
      this.right = 0;
      this.wPressed = false;
      this.aPressed = false;
      this.sPressed = false;
      this.dPressed = false;
      this.$http.put(`${API_URL}/api/stop`).then(() => { }, (response) => {
        console.log(response);
        this.errorMessage = response;
      });
    },
    setSpeed() {
      this.$http.put(`${API_URL}/api/speed`, {
        left: this.left * 10,
        right: this.right * 10,
      }).then(() => { }, (response) => {
        this.errorMessage = response;
      });
    },
    reset() {
      this.enabled = true;
      this.speed = 10;
      this.balance = 0;
      this.wPressed = false;
      this.aPressed = false;
      this.sPressed = false;
      this.dPressed = false;
      this.$http.put(`${API_URL}/api/reset`).then(() => { }, (response) => {
        this.errorMessage = response;
      });
    },
    toggleLock() {
      this.locked = !this.locked;
    },
    keyPressed(event) {
      if (event.key === 'w') {
        this.wPressed = true;
      } else if (event.key === 'a') {
        this.aPressed = true;
      } else if (event.key === 's') {
        this.sPressed = true;
      } else if (event.key === 'd') {
        this.dPressed = true;
      } else if (event.key === 'space') {
        this.toggleEnabled();
        return;
      }
      this.calculateSpeeds();
    },
    keyReleased(event) {
      if (event.key === 'w') {
        this.wPressed = false;
      } else if (event.key === 'a') {
        this.aPressed = false;
      } else if (event.key === 's') {
        this.sPressed = false;
      } else if (event.key === 'd') {
        this.dPressed = false;
      } else if (event.key === 'space') {
        return;
      }
      this.calculateSpeeds();
    },
    calculateSpeeds() {
      let left = 0;
      let right = 0;
      if (this.wPressed) {
        left += 10;
        right += 10;
      }
      if (this.sPressed) {
        left -= 10;
        right -= 10;
      }
      if (this.aPressed) {
        left -= 10;
        right += 10;
      }
      if (this.dPressed) {
        left += 10;
        right -= 10;
      }

      if (left > 10) {
        left = 10;
      }
      if (left < -10) {
        left = -10;
      }
      if (right > 10) {
        right = 10;
      }
      if (right < -10) {
        right = -10;
      }

      this.$set(this, 'left', left);
      this.$set(this, 'right', right);
    },
  },
  watch: {
    left() {
      this.setSpeed();
    },
    right() {
      this.setSpeed();
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.controls {
  position: absolute;
  bottom: 0;
  width: 100%;
  margin-bottom: 50px;
}

.controls form {
  margin: auto;
  width: 50%;
}

main {
  height: 100vh;
  width: 100vw;
  background-color: #f5f5f5;
}
</style>
