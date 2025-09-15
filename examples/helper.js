const path = require('path')

/**
 * Validates and sanitizes user input to prevent path traversal attacks
 * @param {string} userInput - The user-provided path input
 * @returns {string} - The validated and sanitized path, or 'Access denied' if invalid
 */
exports.validatePath = (userInput, root = process.cwd()) => {
  // Check for null byte injection
  if (userInput.indexOf('\0') !== -1) {
    return 'Access denied'
  }

  // Construct the full path
  const pathString = path.join(root, userInput)

  // Final security check: ensure the path is within the root directory
  if (path.resolve(pathString).indexOf(path.resolve(root).toString()) !== 0) {
    return 'Access denied'
  }

  return pathString
}
