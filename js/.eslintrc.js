function kv (v, keys) {
  return keys.reduce((a, k) => (a[k] = v, a), {});
}

const offs = kv('off', [
  'camelcase',
  'class-methods-use-this',
  'import/extensions',
  'import/named',
  'import/no-cycle',
  'import/no-named-as-default',
  'import/no-unresolved',
  'import/prefer-default-export',
  'lines-between-class-members',
  'max-len',
  'no-alert',
  'no-extra-bind',
  'no-param-reassign',
  'no-process-env',
  'no-restricted-globals',
  'no-return-assign',
  'no-sequences',
  'no-shadow',
  'no-underscore-dangle',
  'no-unused-expressions',
  'object-curly-newline',
  'prefer-arrow-callback',
  'require-jsdoc',
  'typescript/explicit-function-return-type',
  'typescript/explicit-member-accessibility',
  'typescript/member-delimiter-style',
  'yoda',
]);

const errors = kv('error', [
  'block-scoped-var',
  'default-case',
  'dot-notation',
  'guard-for-in',
  'no-cond-assign',
  'no-console',
  'no-constant-condition',
  'no-div-regex',
  'no-else-return',
  'no-eq-null',
  'no-extend-native',
  'no-floating-decimal',
  'no-iterator',
  'no-loop-func',
  'no-multi-str',
  'no-proto',
  'no-self-compare',
  'no-throw-literal',
  'no-unused-vars',
  'radix',
  'react-hooks/rules-of-hooks',
  'semi',
  'typescript/no-unused-vars',
  'typescript/no-use-before-define',
]);

const details = {
  'comma-dangle': [
    'error',
    'always-multiline',
  ],
  'curly': [
    'error',
    'all',
  ],
  'indent': [
    'error', 2, {
      'MemberExpression': 1,
      'SwitchCase': 0,
    },
  ],
  'no-multi-spaces': [
    'error',
    {
      exceptions: {
        AssignmentExpression: true,
        ImportDeclaration: true,
        Property: true,
        VariableDeclarator: true,
      },
    },
  ],
  'space-before-function-paren': [
    'error',
    'always',
  ],
  'wrap-iife': [
    'error',
    'inside',
  ],
};

module.exports = {
  env: {
    es6: true,
    node: true,
  },
  extends: [
    'airbnb-base',
    'eslint:recommended',
    'standard-jsx',
    'typescript',
  ],
  globals: {
    APP_ENV: true,
    document: true,
    window: true,
  },
  overrides: {
    files: ['**/*.ts', '**/*.tsx'],
    rules: {
      'no-undef': 'off',
      'no-unused-vars': 'off',
    },
  },
  parser: 'typescript-eslint-parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 6,
    sourceType: 'module',
  },
  plugins: [
    'no-storage',
    'react-hooks',
  ],
  root: true,
  rules: {
    ...details,
    ...errors,
    ...offs,
  },
  settings: {
    'import/extensions': ['.js', '.jsx', '.ts', '.tsx'],
    'import/resolver': {
      node: {
        extensions: ['.js', '.jsx', '.ts', '.tsx'],
      },
    },
    'react': {
      'pragma': 'h',
    },
  },
};
