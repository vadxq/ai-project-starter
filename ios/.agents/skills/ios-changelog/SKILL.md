---
name: ios-changelog
description: Create or update ios changelog.md from confirmed changes when asked to write release notes or maintain a changelog; initialize the file when absent.
---

# 编写更新日志

在所属项目目录操作，只维护该目录下的小写 `changelog.md`。先读取本端 AGENTS.md 和 README.md，确认日志语言、现有格式和本次请求范围。

1. 结合用户说明、实际文件内容与 `git --no-pager diff` 核实本次已完成的改动。仓库存在其他未提交修改时，区分它们与当前任务；未确认的历史不要写成新增功能。
2. 文件不存在时，创建 `# Changelog` 和 `## [Unreleased]`。有实际变更才增加条目；没有足够依据时保留空的 Unreleased，并说明缺少变更信息。
3. 文件已存在时，沿用原有语言和结构，将本次内容合并到未发布区，保留已发布历史。同一项修改合并描述，避免重复追加。
4. 新日志按需要使用 Added、Changed、Fixed、Removed 分类，只保留有条目的分类。简短描述用户或维护者可感知的结果；涉及破坏性变更时写明影响与必要迁移方式。
5. 版本号和发布日期仅使用已确认的信息。测试结果仅记录实际执行的结论。排除临时记录、缓存、凭证、个人路径和生成文件噪声。
6. 检查 Markdown、重复条目和文件改动范围；新增文件要直接检查内容，不能仅依赖 Git diff。交付时说明日志路径和本次增加/合并的内容。仅维护更新日志，不自动提交、打 tag 或发布。
