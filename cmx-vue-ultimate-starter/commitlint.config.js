// @ts-check
/** @type {import('@commitlint/types').UserConfig} */
export default {
  // 忽略包含 "init" 的提交（如初始化提交）
  ignores: [(commit) => commit.includes('init')],

  // 继承基础规范
  extends: ['@commitlint/config-conventional'],

  rules: {
    // 格式规则
    'body-leading-blank': [2, 'always'], // 正文前空行
    'footer-leading-blank': [1, 'always'], // 脚注前空行
    'header-max-length': [2, 'always', 108], // 标题最大长度
    'type-case': [2, 'always', 'lower-case'], // 类型小写

    // 内容规则
    'subject-empty': [2, 'never'], // 禁止空描述
    'type-empty': [2, 'never'], // 禁止空类型

    // 类型枚举（带完整注释）
    'type-enum': [
      2,
      'always',
      [
        'feat', // 新增功能
        'fix', // Bug修复
        'perf', // 性能优化
        'style', // 代码样式
        'docs', // 文档更新
        'test', // 测试相关
        'refactor', // 代码重构
        'build', // 构建系统
        'ci', // CI配置
        'chore', // 日常维护
        'revert', // 回滚提交
        'wip', // 开发中代码
        'workflow', // 工作流改进
        'types', // 类型定义
        'release', // 版本发布
        'security', // 安全相关
        'i18n', // 国际化
        'a11y', // 可访问性
        'db', // 数据库变更
        'deps', // 依赖更新
      ],
    ],
  },
}
