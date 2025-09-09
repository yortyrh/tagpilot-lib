import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

// Platform configurations
const platforms = [
  { dir: 'android-arm-eabi', name: 'android-arm-eabi', os: 'android', arch: 'arm', libc: 'eabi' },
  { dir: 'android-arm64', name: 'android-arm64', os: 'android', arch: 'arm64', libc: null },
  { dir: 'darwin-arm64', name: 'darwin-arm64', os: 'darwin', arch: 'arm64', libc: null },
  { dir: 'darwin-x64', name: 'darwin-x64', os: 'darwin', arch: 'x64', libc: null },
  { dir: 'freebsd-x64', name: 'freebsd-x64', os: 'freebsd', arch: 'x64', libc: null },
  { dir: 'linux-arm-gnueabihf', name: 'linux-arm-gnueabihf', os: 'linux', arch: 'arm', libc: 'gnueabihf' },
  { dir: 'linux-arm64-gnu', name: 'linux-arm64-gnu', os: 'linux', arch: 'arm64', libc: 'gnu' },
  { dir: 'linux-arm64-musl', name: 'linux-arm64-musl', os: 'linux', arch: 'arm64', libc: 'musl' },
  { dir: 'linux-x64-gnu', name: 'linux-x64-gnu', os: 'linux', arch: 'x64', libc: 'gnu' },
  { dir: 'linux-x64-musl', name: 'linux-x64-musl', os: 'linux', arch: 'x64', libc: 'musl' },
  { dir: 'win32-arm64-msvc', name: 'win32-arm64-msvc', os: 'win32', arch: 'arm64', libc: 'msvc' },
  { dir: 'win32-ia32-msvc', name: 'win32-ia32-msvc', os: 'win32', arch: 'ia32', libc: 'msvc' },
  { dir: 'win32-x64-msvc', name: 'win32-x64-msvc', os: 'win32', arch: 'x64', libc: 'msvc' },
  { dir: 'wasm32-wasip1-threads', name: 'wasm32-wasip1-threads', os: 'wasip1', arch: 'threads', libc: null },
  { dir: 'wasm32-wasi', name: 'wasm32-wasi', os: 'wasi', arch: 'wasi', libc: null },
]

// Base package.json template
// Copy From the original one: ./package.json
const basePackageJson = JSON.parse(fs.readFileSync('./package.json', 'utf8'))

// Generate package.json for each platform
platforms.forEach((platform) => {
  // create file structure for each platform
  const platformDir = path.join('npm', platform.dir)
  fs.mkdirSync(platformDir, { recursive: true })

  const packageJson = {
    ...basePackageJson,
    name: `@yortyrh/tagpilot-lib-${platform.name}`,
    description: `${basePackageJson.description} (${platform.os} ${platform.arch}${platform.libc ? ` ${platform.libc}` : ''})`,
    os: [platform.os],
    cpu: [platform.arch],
    ...(platform.libc && { libc: [platform.libc] }),
  }

  const packagePath = path.join('npm', platform.dir, 'package.json')
  fs.writeFileSync(packagePath, JSON.stringify(packageJson, null, 2))
  console.log(`Generated package.json for ${platform.dir}`)
})

console.log('All package.json files generated successfully!')
