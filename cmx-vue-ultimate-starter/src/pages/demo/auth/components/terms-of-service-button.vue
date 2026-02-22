<script lang="ts" setup>
import { ref } from 'vue'

const isOpen = ref(false)
</script>

<template>
  <UiDialog v-model:open="isOpen">
    <UiDialogTrigger as-child>
      <UiButton
        variant="link"
        class="px-0 font-normal underline text-muted-foreground hover:text-primary"
      >
        服务条款
      </UiButton>
    </UiDialogTrigger>
    <UiDialogContent class="max-w-2xl max-h-[80vh]">
      <UiDialogHeader>
        <UiDialogTitle class="text-2xl font-bold">服务条款</UiDialogTitle>
        <UiDialogDescription> 请仔细阅读以下服务条款 </UiDialogDescription>
      </UiDialogHeader>

      <UiScrollArea class="h-[60vh] pr-4">
        <div class="space-y-4 text-sm">
          <section>
            <h3 class="font-semibold text-base mb-2">1. 服务说明</h3>
            <p class="text-muted-foreground leading-relaxed">
              欢迎使用Riusky软件提供的服务。本服务条款是您与Riusky软件之间的法律协议，适用于您使用Riusky软件提供的所有产品和服务。使用我们的服务即表示您同意接受本服务条款的约束。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">2. 账号注册</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              2.1
              您在注册账号时应提供真实、准确、完整的个人信息，并及时更新您的信息以保持其真实性、准确性和完整性。
            </p>
            <p class="text-muted-foreground leading-relaxed mb-2">
              2.2
              您应妥善保管账号和密码，对通过您的账号进行的所有活动负责。如发现账号被未经授权使用，应立即通知我们。
            </p>
            <p class="text-muted-foreground leading-relaxed">
              2.3 一个手机号/邮箱只能注册一个账号。禁止恶意注册多个账号。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">3. 用户行为规范</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              3.1 您在使用本服务时应遵守中华人民共和国相关法律法规，不得利用本服务从事以下行为：
            </p>
            <ul class="list-disc list-inside space-y-1 text-muted-foreground ml-4">
              <li>发布、传播违反法律法规的内容</li>
              <li>侵犯他人知识产权、商业秘密等合法权益</li>
              <li>发布虚假信息、垃圾信息或恶意营销信息</li>
              <li>传播病毒、木马或其他恶意程序</li>
              <li>干扰或破坏本服务的正常运行</li>
            </ul>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">4. 知识产权</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              4.1
              本服务中包含的所有内容，包括但不限于文字、图片、音频、视频、软件、程序、版面设计等，其知识产权均归Riusky软件或相关权利人所有。
            </p>
            <p class="text-muted-foreground leading-relaxed">
              4.2 未经Riusky软件书面许可，您不得以任何形式使用本服务中的内容。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">5. 隐私保护</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              5.1 我们重视用户隐私保护，会采取合理的安全措施保护您的个人信息。
            </p>
            <p class="text-muted-foreground leading-relaxed">
              5.2 我们收集和使用您的个人信息将遵循我们的隐私政策。请您仔细阅读我们的隐私政策。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">6. 免责声明</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              6.1 本服务按"现状"提供，我们不对服务的及时性、安全性、准确性做出任何明示或暗示的保证。
            </p>
            <p class="text-muted-foreground leading-relaxed">
              6.2 因不可抗力、系统故障、网络中断等原因导致的服务中断或数据丢失，我们不承担责任。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">7. 服务变更和终止</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              7.1 我们有权根据业务发展需要修改、暂停或终止部分或全部服务，并会提前通知您。
            </p>
            <p class="text-muted-foreground leading-relaxed">
              7.2 如您违反本服务条款，我们有权暂停或终止向您提供服务。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">8. 协议修改</h3>
            <p class="text-muted-foreground leading-relaxed">
              我们有权随时修改本服务条款，修改后的条款将在本页面公布。您继续使用本服务即表示您接受修改后的服务条款。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">9. 法律适用和争议解决</h3>
            <p class="text-muted-foreground leading-relaxed mb-2">
              9.1 本服务条款的解释、效力及纠纷的解决，适用中华人民共和国法律。
            </p>
            <p class="text-muted-foreground leading-relaxed">
              9.2
              若双方发生争议，应友好协商解决；协商不成的，任何一方可向Riusky软件所在地人民法院提起诉讼。
            </p>
          </section>

          <section>
            <h3 class="font-semibold text-base mb-2">10. 联系我们</h3>
            <p class="text-muted-foreground leading-relaxed">
              如您对本服务条款有任何疑问，请联系我们：support@haiyuan-software.com
            </p>
          </section>

          <div class="mt-6 pt-4 border-t text-xs text-muted-foreground">
            <p>最后更新时间：2025年10月21日</p>
            <p class="mt-1">版本号：v1.0</p>
          </div>
        </div>
      </UiScrollArea>

      <UiDialogFooter>
        <UiButton @click="isOpen = false" class="w-full sm:w-auto"> 我已阅读并同意 </UiButton>
      </UiDialogFooter>
    </UiDialogContent>
  </UiDialog>
</template>
