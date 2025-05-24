export default {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'header-max-length': [0, 'never', 100],
    'body-max-line-length': [0, 'never', 100],
  }
};
