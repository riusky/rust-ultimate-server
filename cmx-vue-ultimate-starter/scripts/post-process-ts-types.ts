/**
 * 后处理 ts-rs 生成的 TypeScript 类型文件
 * 1. 转换 filter 目录中的类型格式（tagged union -> $eq 对象格式）
 * 2. 使用 prettier 格式化
 * 3. 自动生成 index.ts
 *
 * 使用: npx tsx scripts/post-process-ts-types.ts
 */

/// <reference types="node" />

import { execSync } from 'node:child_process'
import * as fs from 'node:fs'
import * as path from 'node:path'
import { fileURLToPath } from 'node:url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const GENERATED_DIR = path.resolve(__dirname, '../src/services/types')

// region: --- Filter 类型转换配置

// 操作符映射：Rust enum variant -> JSON 操作符
const OPERATOR_MAP: Record<string, string> = {
  Eq: '$eq',
  Not: '$not',
  In: '$in',
  NotIn: '$notIn',
  Lt: '$lt',
  Lte: '$lte',
  Gt: '$gt',
  Gte: '$gte',
  Null: '$null',
  Empty: '$empty',
  // String specific
  Contains: '$contains',
  NotContains: '$notContains',
  ContainsAny: '$containsAny',
  NotContainsAny: '$notContainsAny',
  ContainsAll: '$containsAll',
  StartsWith: '$startsWith',
  NotStartsWith: '$notStartsWith',
  StartsWithAny: '$startsWithAny',
  NotStartsWithAny: '$notStartsWithAny',
  EndsWith: '$endsWith',
  NotEndsWith: '$notEndsWith',
  EndsWithAny: '$endsWithAny',
  NotEndsWithAny: '$notEndsWithAny',
  ContainsCi: '$containsCi',
  NotContainsCi: '$notContainsCi',
  StartsWithCi: '$startsWithCi',
  NotStartsWithCi: '$notStartsWithCi',
  EndsWithCi: '$endsWithCi',
  NotEndsWithCi: '$notEndsWithCi',
  Ilike: '$ilike',
}

// 类型映射：ts-rs 生成的类型 -> 实际使用的类型
const TYPE_MAP: Record<string, string> = {
  bigint: 'number',
  unknown: 'unknown',
}

// endregion: --- Filter 类型转换配置

// region: --- Filter 类型转换函数

/**
 * 解析 OpVal 类型的 tagged union 格式
 * 输入: { "Eq": bigint } | { "Not": bigint } | ...
 * 输出: [{ name: "Eq", type: "bigint" }, { name: "Not", type: "bigint" }, ...]
 */
function parseTaggedUnion(typeStr: string): Array<{ name: string; type: string }> {
  const results: Array<{ name: string; type: string }> = []

  // 匹配 { "Name": Type } 模式
  const pattern = /\{\s*"?(\w+)"?\s*:\s*([^}]+)\s*\}/g
  let match: RegExpExecArray | null

  while ((match = pattern.exec(typeStr)) !== null) {
    const name = match[1]
    let type = match[2].trim()

    // 处理 Array<T> 格式
    if (type.startsWith('Array<')) {
      const innerType = type.slice(6, -1)
      const mappedInner = TYPE_MAP[innerType] || innerType
      type = `${mappedInner}[]`
    } else {
      type = TYPE_MAP[type] || type
    }

    results.push({ name, type })
  }

  return results
}

/**
 * 转换 OpVal 类型文件
 * 从 tagged union 格式转换为 $eq 对象格式
 */
function transformOpValFile(filePath: string): void {
  const content = fs.readFileSync(filePath, 'utf-8')
  const fileName = path.basename(filePath, '.ts')

  // 跳过 OpVals* 包装类型（它们只是 Array<OpVal*>）
  if (fileName.startsWith('OpVals')) {
    // 生成简单的类型别名
    const innerType = fileName.replace('OpVals', 'OpVal')
    const newContent = `// Auto-generated from ts-rs, transformed to modql JSON format
export type ${fileName} = ${innerType} | ${innerType}[]
`
    fs.writeFileSync(filePath, newContent)
    return
  }

  // 解析 tagged union
  const operators = parseTaggedUnion(content)

  if (operators.length === 0) {
    console.log(`    跳过: ${fileName} (无法解析)`)
    return
  }

  // 生成 interface 格式
  const fields = operators
    .map(({ name, type }) => {
      const jsonOp = OPERATOR_MAP[name]
      if (!jsonOp) {
        console.log(`    警告: 未知操作符 ${name}`)
        return null
      }
      return `  ${jsonOp}?: ${type}`
    })
    .filter(Boolean)
    .join('\n')

  const newContent = `// Auto-generated from ts-rs, transformed to modql JSON format
export interface ${fileName} {
${fields}
}
`

  fs.writeFileSync(filePath, newContent)
}

/**
 * 处理 filter 目录：转换所有 OpVal* 类型
 */
function processFilterDir(): void {
  const filterDir = path.join(GENERATED_DIR, 'filter')

  if (!fs.existsSync(filterDir)) {
    console.log('  filter 目录不存在，跳过')
    return
  }

  console.log('转换 filter 类型格式:')

  const files = fs.readdirSync(filterDir)
  const opValFiles = files.filter((f) => f.startsWith('OpVal') && f.endsWith('.ts'))

  // 先处理 OpVal* (非 OpVals*)
  for (const file of opValFiles.filter((f) => !f.startsWith('OpVals'))) {
    const filePath = path.join(filterDir, file)
    transformOpValFile(filePath)
    console.log(`  转换: ${file}`)
  }

  // 再处理 OpVals*
  for (const file of opValFiles.filter((f) => f.startsWith('OpVals'))) {
    const filePath = path.join(filterDir, file)
    transformOpValFile(filePath)
    console.log(`  转换: ${file}`)
  }
}

// endregion: --- Filter 类型转换函数

// 获取目录下所有 .ts 文件（排除 index.ts）
function getTypeFiles(dir: string): string[] {
  const files = fs.readdirSync(dir)
  return files
    .filter((f) => f.endsWith('.ts') && f !== 'index.ts')
    .map((f) => f.replace('.ts', ''))
}

// 获取所有子目录
function getSubDirs(dir: string): string[] {
  const items = fs.readdirSync(dir, { withFileTypes: true })
  return items.filter((item) => item.isDirectory()).map((item) => item.name)
}

// 生成子目录的 index.ts
function generateSubDirIndex(subDir: string): void {
  const fullPath = path.join(GENERATED_DIR, subDir)
  const typeFiles = getTypeFiles(fullPath)

  if (typeFiles.length === 0) {
    console.log(`  跳过空目录: ${subDir}/`)
    return
  }

  const exports = typeFiles.map((name) => `export type { ${name} } from './${name}'`).join('\n')

  const content = `// Auto-generated index file
${exports}
`

  fs.writeFileSync(path.join(fullPath, 'index.ts'), content)
  console.log(`  生成: ${subDir}/index.ts (${typeFiles.length} types)`)
}

// 生成根目录的 index.ts
function generateRootIndex(): void {
  const subDirs = getSubDirs(GENERATED_DIR)

  const imports: string[] = []

  for (const subDir of subDirs) {
    const fullPath = path.join(GENERATED_DIR, subDir)
    const typeFiles = getTypeFiles(fullPath)

    if (typeFiles.length === 0) continue

    const typeList = typeFiles.join(', ')
    // Capitalize first letter for comment
    const comment = `${subDir.charAt(0).toUpperCase() + subDir.slice(1)} types`
    imports.push(`// ${comment}`)
    imports.push(`export type { ${typeList} } from './${subDir}/index'`)
    imports.push('')
  }

  const content = `// Auto-generated index file
${imports.join('\n')}`

  fs.writeFileSync(path.join(GENERATED_DIR, 'index.ts'), content)
  console.log(`  生成: index.ts`)
}

// 格式化所有 .ts 文件
function formatFiles(): void {
  console.log('\n格式化 TypeScript 文件...')
  try {
    execSync(`npx prettier --write "${GENERATED_DIR}/**/*.ts"`, {
      stdio: 'inherit',
    })
  } catch {
    console.warn('  警告: prettier 格式化失败，跳过')
  }
}

// 主函数
function main(): void {
  console.log('后处理 ts-rs 生成的类型文件...\n')

  // 1. 转换 filter 目录中的类型格式
  processFilterDir()

  // 2. 生成子目录的 index.ts
  console.log('\n生成 index.ts 文件:')
  const subDirs = getSubDirs(GENERATED_DIR)
  for (const subDir of subDirs) {
    generateSubDirIndex(subDir)
  }

  // 3. 生成根目录的 index.ts
  generateRootIndex()

  // 4. 格式化
  formatFiles()

  console.log('\n完成!')
}

main()
