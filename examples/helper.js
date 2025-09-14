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

  // Validate that input contains only alphanumeric characters
  if (!/^[a-z0-9]+$/.test(userInput)) {
    return 'Access denied'
  }

  // Normalize the path and remove any leading directory traversal sequences
  const safeInput = path.normalize(userInput).replace(/^(\.\.(\/|\\|$))+/, '')

  // Construct the full path
  const pathString = path.join(root, safeInput)

  // Final security check: ensure the path is still within the root directory
  if (pathString.indexOf(root) !== 0) {
    return 'Access denied'
  }

  return pathString
}
