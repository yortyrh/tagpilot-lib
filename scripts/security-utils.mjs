import fs from 'fs'
import path from 'path'

/**
 * Security utilities to prevent path traversal and other file system vulnerabilities
 * Based on CWE-23 (Relative Path Traversal) and related security concerns
 */

/**
 * Validates if a file name is safe to use
 * @param {string} fileName - The file name to validate
 * @returns {boolean} - True if the file name is safe
 */
export function isValidFileName(fileName) {
  if (!fileName || typeof fileName !== 'string') {
    return false
  }

  // Check for path traversal patterns and other dangerous characters
  const dangerousPatterns = [
    /\.\./, // Parent directory traversal
    /\.\.\\/, // Windows parent directory traversal
    /\.\.\//, // Unix parent directory traversal
    /[<>:"|?*]/, // Windows reserved characters
    // eslint-disable-next-line no-control-regex
    /[\x00-\x1f]/, // Control characters
    /^\./, // Hidden files (starting with dot)
    /\/$/, // Directory paths (ending with slash)
    /\\$/, // Windows directory paths (ending with backslash)
  ]

  return !dangerousPatterns.some((pattern) => pattern.test(fileName))
}

/**
 * Sanitizes a file name by removing dangerous characters
 * @param {string} fileName - The file name to sanitize
 * @returns {string} - The sanitized file name
 */
export function sanitizeFileName(fileName) {
  if (!fileName || typeof fileName !== 'string') {
    return ''
  }

  // Remove any potentially dangerous characters
  return (
    fileName
      // eslint-disable-next-line no-control-regex
      .replace(/[<>:"|?*\x00-\x1f]/g, '')
      .replace(/\.\./g, '')
      .replace(/^\.+/, '') // Remove leading dots
      .replace(/[/\\]+$/, '')
  ) // Remove trailing slashes/backslashes
}

/**
 * Validates and resolves a file path safely
 * @param {string} filePath - The file path to validate
 * @param {string} baseDir - The base directory to resolve against
 * @returns {string|null} - The resolved safe path or null if invalid
 */
export function validateAndResolvePath(filePath, baseDir) {
  if (!filePath || typeof filePath !== 'string') {
    return null
  }

  try {
    // Resolve the base directory
    const resolvedBaseDir = path.resolve(baseDir)

    // Resolve the file path
    const resolvedFilePath = path.resolve(resolvedBaseDir, filePath)

    // Ensure the resolved path is within the base directory
    if (!resolvedFilePath.startsWith(resolvedBaseDir)) {
      console.warn(`Path traversal detected: ${filePath}`)
      return null
    }

    // Check if the file exists and is a file (not directory)
    const stats = fs.statSync(resolvedFilePath)
    if (!stats.isFile()) {
      console.warn(`Path is not a file: ${filePath}`)
      return null
    }

    return resolvedFilePath
  } catch (error) {
    console.warn(`Invalid file path: ${filePath} - ${error.message}`)
    return null
  }
}

/**
 * Safely reads a file with path validation
 * @param {string} filePath - The file path to read
 * @param {string} baseDir - The base directory to resolve against
 * @param {string} encoding - The encoding to use (optional)
 * @returns {Buffer|string|null} - The file content or null if invalid
 */
export function safeReadFile(filePath, baseDir, encoding = null) {
  const safePath = validateAndResolvePath(filePath, baseDir)
  if (!safePath) {
    return null
  }

  try {
    if (encoding) {
      return fs.readFileSync(safePath, encoding)
    } else {
      return fs.readFileSync(safePath)
    }
  } catch (error) {
    console.error(`Error reading file ${filePath}:`, error.message)
    return null
  }
}

/**
 * Safely reads a file asynchronously with path validation
 * @param {string} filePath - The file path to read
 * @param {string} baseDir - The base directory to resolve against
 * @param {string} encoding - The encoding to use (optional)
 * @returns {Promise<Buffer|string|null>} - The file content or null if invalid
 */
export async function safeReadFileAsync(filePath, baseDir, encoding = null) {
  const safePath = validateAndResolvePath(filePath, baseDir)
  if (!safePath) {
    return null
  }

  try {
    if (encoding) {
      return await fs.promises.readFile(safePath, encoding)
    } else {
      return await fs.promises.readFile(safePath)
    }
  } catch (error) {
    console.error(`Error reading file ${filePath}:`, error.message)
    return null
  }
}

/**
 * Validates command line arguments for file paths
 * @param {string[]} args - Command line arguments
 * @param {string} baseDir - The base directory to resolve against
 * @returns {string[]} - Array of validated file paths
 */
export function validateFileArguments(args, baseDir) {
  const validPaths = []

  for (const arg of args) {
    if (isValidFileName(path.basename(arg))) {
      const safePath = validateAndResolvePath(arg, baseDir)
      if (safePath) {
        validPaths.push(safePath)
      }
    } else {
      console.warn(`Skipping potentially unsafe file path: ${arg}`)
    }
  }

  return validPaths
}
