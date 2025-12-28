/**
 * 后处理 ts-rs 生成的 TypeScript 类型文件
 * 1. 使用 prettier 格式化
 * 2. 自动生成 index.ts
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

  // 1. 生成子目录的 index.ts
  console.log('生成 index.ts 文件:')
  const subDirs = getSubDirs(GENERATED_DIR)
  for (const subDir of subDirs) {
    generateSubDirIndex(subDir)
  }

  // 2. 生成根目录的 index.ts
  generateRootIndex()

  // 3. 格式化
  formatFiles()

  console.log('\n完成!')
}

main()
