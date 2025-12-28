import { useI18n } from 'vue-i18n'

/**
 * Get localized error message from business error code
 *
 * @param bizCode - Business error code from backend (e.g., "AUTH#001")
 * @returns Localized error message
 *
 * @example
 * ```ts
 * const { getBizErrorMessage } = useBizError()
 *
 * // In error handler
 * if (response.error?.biz_code) {
 *   const message = getBizErrorMessage(response.error.biz_code)
 *   toast.error(message)
 * }
 * ```
 */
export function useBizError() {
  const { t, te } = useI18n()

  function getBizErrorMessage(bizCode: string | undefined | null): string {
    if (!bizCode) {
      return t('bizErrors.UNKNOWN')
    }

    // Check if translation exists for this biz_code
    const key = `bizErrors.${bizCode}`
    if (te(key)) {
      return t(key)
    }

    // Fallback to UNKNOWN
    return t('bizErrors.UNKNOWN')
  }

  return {
    getBizErrorMessage,
  }
}
