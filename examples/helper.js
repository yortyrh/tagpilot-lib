const path = require('path')

/**
 * Validates and sanitizes user input to prevent path traversal attacks
 * @param {string} userInput - The user-provided path input
 * @param {string} root - The root directory to validate against
 * @returns {string|null} - The validated and sanitized path, or null if invalid
 */
exports.validatePath = (userInput, root = process.cwd()) => {
  // Validate input type and basic format first
  if (!userInput || typeof userInput !== 'string') {
    return null
  }

  // Check for null byte injection
  if (userInput.indexOf('\0') !== -1) {
    return null
  }

  // Check for dangerous patterns that could lead to path traversal
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

  if (dangerousPatterns.some((pattern) => pattern.test(userInput))) {
    return null
  }

  // Normalize the path to resolve any remaining issues
  const normalizedInput = path.normalize(userInput)

  // Double-check normalized input for traversal patterns
  if (normalizedInput.includes('..') || normalizedInput.startsWith('.')) {
    return null
  }

  // Construct the full path
  const pathString = path.join(root, normalizedInput)

  // Resolve both paths to absolute paths for proper comparison
  const resolvedPath = path.resolve(pathString)
  const resolvedRoot = path.resolve(root)

  // Critical security check: ensure the resolved path is within the root directory
  // This prevents directory traversal attacks by ensuring the resolved path
  // starts with the resolved root path followed by a path separator
  if (!resolvedPath.startsWith(resolvedRoot + path.sep) && resolvedPath !== resolvedRoot) {
    return null
  }

  return pathString
}
