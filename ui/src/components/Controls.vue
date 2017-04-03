<template>
  <main>
    <section class="alert-box -error" v-if="errorMessage !== false">
      <p>{{ errorMessage }}</p>
      <a class="close" @click="errorMessage = false" href="#">×</a>
    </section>
    <section>
      <div class="controls">
        <form>
          <div grid>
            <div column>
              <label>
                <span>Left</span>
                <input type="range" id="left" v-model="left" max="10" min="-10" value="0">
              </label>
            </div>
            <div column>
              <label>
                <span>Right</span>
                <input type="range" id="right" v-model="right" max="10" min="-10" value="0">
              </label>
            </div>
          </div>
          <div>
            <button @click="toggleStopped">
              <span v-if="stopped">Start</span>
              <span v-else>Stop</span>
            </button>
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
  const API_URL = `${process.env.BASE_URL}/api`;

  export default {
    name: 'controls',
    data() {
      return {
        left: '0',
        right: '0',
        errorMessage: false,
        stopped: true,
        enabled: true,
        wPressed: false,
        aPressed: false,
        sPressed: false,
        dPressed: false,
        interval: null,
      };
    },
    created() {
      window.addEventListener('keyup', this.keyReleased);
      window.addEventListener('keydown', this.keyPressed);
    },
    beforeDestroy() {
      this.stop();
    },
    methods: {
      errorHandler(response) {
        if (response.body && response.body.error) {
          this.errorMessage = response.body.error;
        } else {
          this.errorMessage = `Unable to connect to ${response.url}`;
        }
      },
      toggleStopped() {
        this.$set(this, 'stopped', !this.stopped);
        if (this.stopped) {
          this.stop();
        } else {
          this.start();
        }
      },
      stop() {
        clearInterval(this.interval);
        this.left = 0;
        this.right = 0;
        this.wPressed = false;
        this.aPressed = false;
        this.sPressed = false;
        this.dPressed = false;
        this.$http.put(`${API_URL}/stop`).then(null, this.errorHandler);
      },
      start() {
        this.setSpeed();
        this.interval = setInterval(this.setSpeed, 50);
      },
      toggleEnabled() {
        this.$set(this, 'enabled', !this.enabled);
        if (this.enabled) {
          this.$http.put(`${API_URL}/enable`).then(null, this.errorHandler);
        } else {
          this.$http.put(`${API_URL}/disable`).then(null, this.errorHandler);
        }
      },
      setSpeed() {
        this.$http.put(`${API_URL}/speed`, {
          left: this.left * 10,
          right: this.right * 10,
        }, { timeout: 200 }).then(null, this.errorHandler);
      },
      reset() {
        this.enabled = true;
        this.left = '0';
        this.right = '0';
        this.wPressed = false;
        this.aPressed = false;
        this.sPressed = false;
        this.dPressed = false;
        this.$http.put(`${API_URL}/reset`).then(null, this.errorHandler);
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
        } else if (event.key === 't') {
          this.toggleStopped();
          return;
        } else if (event.key === 'y') {
          this.toggleEnabled();
          return;
        } else {
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
        } else {
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
