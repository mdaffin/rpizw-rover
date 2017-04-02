var merge = require('webpack-merge')
var prodEnv = require('./prod.env')

module.exports = merge(prodEnv, {
  API_URL: '"http://rpizw-rover.local:3000"',
  NODE_ENV: '"development"'
})
