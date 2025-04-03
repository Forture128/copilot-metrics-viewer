import { useToast, POSITION } from 'vue-toastification'

/**
 * Safe toast options to prevent UI breakage
 */
const safeToastOptions = {
  position: POSITION.BOTTOM_LEFT,
  timeout: 3000,
  closeOnClick: true,
  pauseOnFocusLoss: false,
  pauseOnHover: true,
  draggable: false,
  showCloseButtonOnHover: true,
  hideProgressBar: false,
  icon: true
}

/**
 * More subtle options for non-critical notifications
 */
const subtleToastOptions = {
  ...safeToastOptions,
  timeout: 2000,
  hideProgressBar: true
}

/**
 * Toast service wrapper to provide consistent toast behavior across the application
 */
export class ToastService {
  private static toast = useToast()

  /**
   * Show a success toast notification
   * @param message The message to display
   * @param subtle Whether to use the subtle toast style
   */
  public static success(message: string, subtle = false): void {
    const options = subtle ? subtleToastOptions : safeToastOptions
    this.toast.success(message, options)
  }

  /**
   * Show an error toast notification
   * @param message The message to display
   * @param subtle Whether to use the subtle toast style
   */
  public static error(message: string, subtle = false): void {
    const options = subtle ? subtleToastOptions : safeToastOptions
    this.toast.error(message, options)
  }

  /**
   * Show a warning toast notification
   * @param message The message to display
   * @param subtle Whether to use the subtle toast style
   */
  public static warning(message: string, subtle = false): void {
    // For warnings about missing data, prefer to log to console rather than show a toast
    if (message.includes('No metrics found') || message.includes('No data available')) {
      console.warn(`[ToastService] ${message}`)
      return
    }

    const options = subtle ? subtleToastOptions : safeToastOptions
    this.toast.warning(message, options)
  }

  /**
   * Show an info toast notification
   * @param message The message to display
   * @param subtle Whether to use the subtle toast style
   */
  public static info(message: string, subtle = false): void {
    const options = subtle ? subtleToastOptions : safeToastOptions
    this.toast.info(message, options)
  }

  /**
   * Handle API error with appropriate toast message
   * @param error The error object from API call
   */
  public static handleApiError(error: any): void {
    if (!error) {
      this.error('An unknown error occurred')
      return
    }

    if (error.response) {
      // The request was made and the server responded with a status code outside 2xx range
      const status = error.response.status
      const message = error.response.data?.message || 'An error occurred'

      switch (status) {
        case 401:
          this.error('Authentication failed. Please login again.')
          break
        case 403:
          this.error('You do not have permission to perform this action.')
          break
        case 404:
          this.error('The requested resource was not found.')
          break
        case 422:
          this.error('Validation error. Please check your input.')
          break
        default:
          this.error(`Error: ${message}`)
      }
    } else if (error.request) {
      // The request was made but no response was received
      this.error('No response from server. Please check your connection.')
    } else {
      // Something happened in setting up the request
      this.error(`Request error: ${error.message}`)
    }
  }
}

// Export singleton instance for composition API usage
export default ToastService
