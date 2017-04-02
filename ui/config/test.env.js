var merge = require('webpack-merge')
var devEnv = require('./dev.env')

module.exports = merge(devEnv, {
  API_URL: '""',
  NODE_ENV: '"testing"'
})
