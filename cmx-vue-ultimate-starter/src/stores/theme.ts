import type { Component } from 'vue'

import { MoveHorizontal, UnfoldHorizontal } from 'lucide-vue-next'
import { defineStore } from 'pinia'
import { h } from 'vue'

// export const THEMES = ['zinc', 'red', 'rose', 'orange', 'green', 'blue', 'yellow', 'violet', 'cyberpunk'] as const
// export const THEMES = ['zinc', 'red', 'rose', 'orange', 'green', 'blue', 'yellow', 'violet', 'cyberpunk'] as const
export const THEMES = [
  'default',
  'cyberpunk',
  'amber-minimal',
  'amethyst-haze',
  'bold-tech',
  'bubblegum',
  'caffeine',
  'candyland',
  'catppuccin',
  'claude',
  'claymorphism',
  'clean-slate',
  'cosmic-night',
  'doom-64',
  'elegant-luxury',
  'graphite',
  'kodama-grove',
  'midnight-bloom',
  'mocha-mousse',
  'modern-minimal',
  'mono',
  'nature',
  'neo-brutalism',
  'northern-lights',
  'notebook',
  'ocean-breeze',
  'pastel-dreams',
  'perpetuity',
  'quantum-rose',
  'retro-arcade',
  'soft-pop',
  'solar-dusk',
  'starry-night',
  'sunset-horizon',
  'supabase',
  't3-chat',
  'tangerine',
  'twitter',
  'vercel',
  'vintage-paper',
  'violet-bloom',
] as const

export type Theme = typeof THEMES[number]

export const themes = [
  {
    theme: 'default',
    colors: {
      primary: 'oklch(0.2050 0 0)',
      secondary: 'oklch(0.9700 0 0)',
      muted: 'oklch(0.9700 0 0)',
      accent: 'oklch(0.9700 0 0)',
      destructive: 'oklch(0.5770 0.2450 27.3250)'
    }
  },
  {
    theme: 'cyberpunk',
    colors: {
      primary: 'oklch(0.6726 0.2904 341.4084)',
      secondary: 'oklch(0.9595 0.0200 286.0164)',
      muted: 'oklch(0.9595 0.0200 286.0164)',
      accent: 'oklch(0.8903 0.1739 171.2690)',
      destructive: 'oklch(0.6535 0.2348 34.0370)'
    }
  },
  {
    theme: 'amber-minimal',
    colors: {
      primary: 'oklch(0.7686 0.1647 70.0804)',
      secondary: 'oklch(0.9670 0.0029 264.5419)',
      muted: 'oklch(0.9846 0.0017 247.8389)',
      accent: 'oklch(0.9869 0.0214 95.2774)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'amethyst-haze',
    colors: {
      primary: 'oklch(0.6104 0.0767 299.7335)',
      secondary: 'oklch(0.8957 0.0265 300.2416)',
      muted: 'oklch(0.8906 0.0139 299.7754)',
      accent: 'oklch(0.7889 0.0802 359.9375)',
      destructive: 'oklch(0.6332 0.1578 22.6734)'
    }
  },
  {
    theme: 'bold-tech',
    colors: {
      primary: 'oklch(0.6056 0.2189 292.7172)',
      secondary: 'oklch(0.9618 0.0202 295.1913)',
      muted: 'oklch(0.9691 0.0161 293.7558)',
      accent: 'oklch(0.9319 0.0316 255.5855)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'bubblegum',
    colors: {
      primary: 'oklch(0.6209 0.1801 348.1385)',
      secondary: 'oklch(0.8095 0.0694 198.1863)',
      muted: 'oklch(0.8800 0.0504 212.0952)',
      accent: 'oklch(0.9195 0.0801 87.6670)',
      destructive: 'oklch(0.7091 0.1697 21.9551)'
    }
  },
  {
    theme: 'caffeine',
    colors: {
      primary: 'oklch(0.4341 0.0392 41.9938)',
      secondary: 'oklch(0.9200 0.0651 74.3695)',
      muted: 'oklch(0.9521 0 0)',
      accent: 'oklch(0.9310 0 0)',
      destructive: 'oklch(0.6271 0.1936 33.3390)'
    }
  },
  {
    theme: 'candyland',
    colors: {
      primary: 'oklch(0.8677 0.0735 7.0855)',
      secondary: 'oklch(0.8148 0.0819 225.7537)',
      muted: 'oklch(0.8828 0.0285 98.1033)',
      accent: 'oklch(0.9680 0.2110 109.7692)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'catppuccin',
    colors: {
      primary: 'oklch(0.5547 0.2503 297.0156)',
      secondary: 'oklch(0.8575 0.0145 268.4756)',
      muted: 'oklch(0.9060 0.0117 264.5071)',
      accent: 'oklch(0.6820 0.1448 235.3822)',
      destructive: 'oklch(0.5505 0.2155 19.8095)'
    }
  },
  {
    theme: 'claude',
    colors: {
      primary: 'oklch(0.6171 0.1375 39.0427)',
      secondary: 'oklch(0.9245 0.0138 92.9892)',
      muted: 'oklch(0.9341 0.0153 90.2390)',
      accent: 'oklch(0.9245 0.0138 92.9892)',
      destructive: 'oklch(0.1908 0.0020 106.5859)'
    }
  },
  {
    theme: 'claymorphism',
    colors: {
      primary: 'oklch(0.5854 0.2041 277.1173)',
      secondary: 'oklch(0.8687 0.0043 56.3660)',
      muted: 'oklch(0.9232 0.0026 48.7171)',
      accent: 'oklch(0.9376 0.0260 321.9388)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'clean-slate',
    colors: {
      primary: 'oklch(0.5854 0.2041 277.1173)',
      secondary: 'oklch(0.9276 0.0058 264.5313)',
      muted: 'oklch(0.9670 0.0029 264.5419)',
      accent: 'oklch(0.9299 0.0334 272.7879)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'cosmic-night',
    colors: {
      primary: 'oklch(0.5417 0.1790 288.0332)',
      secondary: 'oklch(0.9174 0.0435 292.6901)',
      muted: 'oklch(0.9580 0.0133 286.1454)',
      accent: 'oklch(0.9221 0.0373 262.1410)',
      destructive: 'oklch(0.6861 0.2061 14.9941)'
    }
  },
  {
    theme: 'doom-64',
    colors: {
      primary: 'oklch(0.5016 0.1887 27.4816)',
      secondary: 'oklch(0.4955 0.0896 126.1858)',
      muted: 'oklch(0.7826 0 0)',
      accent: 'oklch(0.5880 0.0993 245.7394)',
      destructive: 'oklch(0.7076 0.1975 46.4558)'
    }
  },
  {
    theme: 'elegant-luxury',
    colors: {
      primary: 'oklch(0.4650 0.1470 24.9381)',
      secondary: 'oklch(0.9625 0.0385 89.0943)',
      muted: 'oklch(0.9431 0.0068 53.4442)',
      accent: 'oklch(0.9619 0.0580 95.6174)',
      destructive: 'oklch(0.4437 0.1613 26.8994)'
    }
  },
  {
    theme: 'graphite',
    colors: {
      primary: 'oklch(0.4891 0 0)',
      secondary: 'oklch(0.9067 0 0)',
      muted: 'oklch(0.8853 0 0)',
      accent: 'oklch(0.8078 0 0)',
      destructive: 'oklch(0.5594 0.1900 25.8625)'
    }
  },
  {
    theme: 'kodama-grove',
    colors: {
      primary: 'oklch(0.6657 0.1050 118.9078)',
      secondary: 'oklch(0.8532 0.0631 91.1493)',
      muted: 'oklch(0.8532 0.0631 91.1493)',
      accent: 'oklch(0.8361 0.0713 90.3269)',
      destructive: 'oklch(0.7136 0.0981 29.9827)'
    }
  },
  {
    theme: 'midnight-bloom',
    colors: {
      primary: 'oklch(0.5676 0.2021 283.0838)',
      secondary: 'oklch(0.8214 0.0720 249.3482)',
      muted: 'oklch(0.8202 0.0213 91.6163)',
      accent: 'oklch(0.6475 0.0642 117.4260)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'mocha-mousse',
    colors: {
      primary: 'oklch(0.6083 0.0623 44.3588)',
      secondary: 'oklch(0.7473 0.0387 80.5476)',
      muted: 'oklch(0.8502 0.0389 49.0874)',
      accent: 'oklch(0.8502 0.0389 49.0874)',
      destructive: 'oklch(0.2225 0.0098 52.9636)'
    }
  },
  {
    theme: 'modern-minimal',
    colors: {
      primary: 'oklch(0.6231 0.1880 259.8145)',
      secondary: 'oklch(0.9670 0.0029 264.5419)',
      muted: 'oklch(0.9846 0.0017 247.8389)',
      accent: 'oklch(0.9514 0.0250 236.8242)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'mono',
    colors: {
      primary: 'oklch(0.5555 0 0)',
      secondary: 'oklch(0.9702 0 0)',
      muted: 'oklch(0.9702 0 0)',
      accent: 'oklch(0.9702 0 0)',
      destructive: 'oklch(0.5830 0.2387 28.4765)'
    }
  },
  {
    theme: 'nature',
    colors: {
      primary: 'oklch(0.5234 0.1347 144.1672)',
      secondary: 'oklch(0.9571 0.0210 147.6360)',
      muted: 'oklch(0.9370 0.0142 74.4218)',
      accent: 'oklch(0.8952 0.0504 146.0366)',
      destructive: 'oklch(0.5386 0.1937 26.7249)'
    }
  },
  {
    theme: 'neo-brutalism',
    colors: {
      primary: 'oklch(0.6489 0.2370 26.9728)',
      secondary: 'oklch(0.9680 0.2110 109.7692)',
      muted: 'oklch(0.9551 0 0)',
      accent: 'oklch(0.5635 0.2408 260.8178)',
      destructive: 'oklch(0 0 0)'
    }
  },
  {
    theme: 'northern-lights',
    colors: {
      primary: 'oklch(0.6487 0.1538 150.3071)',
      secondary: 'oklch(0.6746 0.1414 261.3380)',
      muted: 'oklch(0.8828 0.0285 98.1033)',
      accent: 'oklch(0.8269 0.1080 211.9627)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'notebook',
    colors: {
      primary: 'oklch(0.4891 0 0)',
      secondary: 'oklch(0.9006 0 0)',
      muted: 'oklch(0.9158 0 0)',
      accent: 'oklch(0.9354 0.0456 94.8549)',
      destructive: 'oklch(0.6627 0.0978 20.0041)'
    }
  },
  {
    theme: 'ocean-breeze',
    colors: {
      primary: 'oklch(0.7227 0.1920 149.5793)',
      secondary: 'oklch(0.9514 0.0250 236.8242)',
      muted: 'oklch(0.9670 0.0029 264.5419)',
      accent: 'oklch(0.9505 0.0507 163.0508)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'pastel-dreams',
    colors: {
      primary: 'oklch(0.7090 0.1592 293.5412)',
      secondary: 'oklch(0.9073 0.0530 306.0902)',
      muted: 'oklch(0.9464 0.0327 307.1745)',
      accent: 'oklch(0.9376 0.0260 321.9388)',
      destructive: 'oklch(0.8077 0.1035 19.5706)'
    }
  },
  {
    theme: 'perpetuity',
    colors: {
      primary: 'oklch(0.5624 0.0947 203.2755)',
      secondary: 'oklch(0.9244 0.0181 196.8450)',
      muted: 'oklch(0.9295 0.0107 196.9723)',
      accent: 'oklch(0.9021 0.0297 201.8915)',
      destructive: 'oklch(0.5732 0.1901 25.5409)'
    }
  },
  {
    theme: 'quantum-rose',
    colors: {
      primary: 'oklch(0.6002 0.2414 0.1348)',
      secondary: 'oklch(0.9230 0.0701 326.1273)',
      muted: 'oklch(0.9429 0.0363 344.2604)',
      accent: 'oklch(0.8766 0.0828 344.8849)',
      destructive: 'oklch(0.5831 0.1911 6.3410)'
    }
  },
  {
    theme: 'retro-arcade',
    colors: {
      primary: 'oklch(0.5924 0.2025 355.8943)',
      secondary: 'oklch(0.6437 0.1019 187.3840)',
      muted: 'oklch(0.6979 0.0159 196.7940)',
      accent: 'oklch(0.5808 0.1732 39.5003)',
      destructive: 'oklch(0.5863 0.2064 27.1172)'
    }
  },
  {
    theme: 'soft-pop',
    colors: {
      primary: 'oklch(0.5106 0.2301 276.9656)',
      secondary: 'oklch(0.7038 0.1230 182.5025)',
      muted: 'oklch(0.9551 0 0)',
      accent: 'oklch(0.7686 0.1647 70.0804)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'solar-dusk',
    colors: {
      primary: 'oklch(0.5553 0.1455 48.9975)',
      secondary: 'oklch(0.8276 0.0752 74.4400)',
      muted: 'oklch(0.9363 0.0218 83.2637)',
      accent: 'oklch(0.9000 0.0500 74.9889)',
      destructive: 'oklch(0.4437 0.1613 26.8994)'
    }
  },
  {
    theme: 'starry-night',
    colors: {
      primary: 'oklch(0.4815 0.1178 263.3758)',
      secondary: 'oklch(0.8567 0.1164 81.0092)',
      muted: 'oklch(0.9202 0.0080 106.5563)',
      accent: 'oklch(0.6896 0.0714 234.0387)',
      destructive: 'oklch(0.2611 0.0376 322.5267)'
    }
  },
  {
    theme: 'sunset-horizon',
    colors: {
      primary: 'oklch(0.7357 0.1641 34.7091)',
      secondary: 'oklch(0.9596 0.0200 28.9029)',
      muted: 'oklch(0.9656 0.0176 39.4009)',
      accent: 'oklch(0.8278 0.1131 57.9984)',
      destructive: 'oklch(0.6122 0.2082 22.2410)'
    }
  },
  {
    theme: 'supabase',
    colors: {
      primary: 'oklch(0.8348 0.1302 160.9080)',
      secondary: 'oklch(0.9940 0 0)',
      muted: 'oklch(0.9461 0 0)',
      accent: 'oklch(0.9461 0 0)',
      destructive: 'oklch(0.5523 0.1927 32.7272)'
    }
  },
  {
    theme: 't3-chat',
    colors: {
      primary: 'oklch(0.5316 0.1409 355.1999)',
      secondary: 'oklch(0.8696 0.0675 334.8991)',
      muted: 'oklch(0.9395 0.0260 331.5454)',
      accent: 'oklch(0.8696 0.0675 334.8991)',
      destructive: 'oklch(0.5248 0.1368 20.8317)'
    }
  },
  {
    theme: 'tangerine',
    colors: {
      primary: 'oklch(0.6397 0.1720 36.4421)',
      secondary: 'oklch(0.9670 0.0029 264.5419)',
      muted: 'oklch(0.9846 0.0017 247.8389)',
      accent: 'oklch(0.9119 0.0222 243.8174)',
      destructive: 'oklch(0.6368 0.2078 25.3313)'
    }
  },
  {
    theme: 'twitter',
    colors: {
      primary: 'oklch(0.6723 0.1606 244.9955)',
      secondary: 'oklch(0.1884 0.0128 248.5103)',
      muted: 'oklch(0.9222 0.0013 286.3737)',
      accent: 'oklch(0.9392 0.0166 250.8453)',
      destructive: 'oklch(0.6188 0.2376 25.7658)'
    }
  },
  {
    theme: 'vercel',
    colors: {
      primary: 'oklch(0 0 0)',
      secondary: 'oklch(0.9400 0 0)',
      muted: 'oklch(0.9700 0 0)',
      accent: 'oklch(0.9400 0 0)',
      destructive: 'oklch(0.6300 0.1900 23.0300)'
    }
  },
  {
    theme: 'vintage-paper',
    colors: {
      primary: 'oklch(0.6180 0.0778 65.5444)',
      secondary: 'oklch(0.8846 0.0302 85.5655)',
      muted: 'oklch(0.9239 0.0190 83.0636)',
      accent: 'oklch(0.8348 0.0426 88.8064)',
      destructive: 'oklch(0.5471 0.1438 32.9149)'
    }
  },
  {
    theme: 'violet-bloom',
    colors: {
      primary: 'oklch(0.5393 0.2713 286.7462)',
      secondary: 'oklch(0.9540 0.0063 255.4755)',
      muted: 'oklch(0.9702 0 0)',
      accent: 'oklch(0.9393 0.0288 266.3680)',
      destructive: 'oklch(0.6290 0.1902 23.0704)'
    }
  },
] as const

export const RADIUS = [0, 0.25, 0.5, 0.75, 1]
export type Radius = typeof RADIUS[number]

export type ContentLayout = 'full' | 'centered'
export const contentLayouts: { label: string, value: ContentLayout, icon: Component }[] = [
  { label: 'Full', value: 'full', icon: h(UnfoldHorizontal) },
  { label: 'Centered', value: 'centered', icon: h(MoveHorizontal) },
]

export const useThemeStore = defineStore('system-config', () => {
  const radius = ref(0.5)
  function setRadius(newRadius: Radius) {
    radius.value = newRadius
  }
  const theme = ref<Theme>('cyberpunk')
  function setTheme(newTheme: Theme) {
    theme.value = newTheme
  }

  const contentLayout = ref<ContentLayout>('centered')
  function setContentLayout(newContentLayout: ContentLayout) {
    contentLayout.value = newContentLayout
  }
  return {
    radius,
    setRadius,

    theme,
    setTheme,

    contentLayout,
    setContentLayout,
  }
}, {
  persist: true,
})
