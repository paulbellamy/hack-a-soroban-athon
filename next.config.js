var fs = require('fs');
/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  env: {
    TOKEN_ID: fs.readFileSync('.soroban/token_id').toString().trim(),
    VOTING_ID: fs.readFileSync('.soroban/voting_id').toString().trim(),
  },
};
