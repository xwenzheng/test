<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, ref } from 'vue'
import { v4 as uuidv4 } from 'uuid'

// 参数 refs，带初始值确保传参完备
const rmb = ref<number>(0)
const remark = ref<string>('')
const type = ref<string>('OfflineRecharge')
const userIdsRaw = ref<string>('')
const result = ref<string>('')
const machine_id = ref<string>('')
const machineResult = ref<string>('')

// 类型选项
const typeOptions = [
  { value: 'Recharge', label: ' 充值' },
  { value: 'OfflineRecharge', label: ' 线下充值' },
  { value: 'TestGold', label: ' 测试金' },
  { value: 'Withdrawal', label: ' 提现' },
  { value: 'ReceivingOrders', label: ' 接单' },
  { value: 'PublishTask', label: ' 发布任务' },
  { value: 'TaskReturn', label: ' 任务退回' },
  { value: 'PlatformReceivingOrders', label: ' 接单 - 平台收入' },
  { value: 'EncounterReceivingOrders', label: ' 被接单' },
  { value: 'MountingFee', label: ' 挂载费用' },
  { value: 'PlatformMountingFee', label: ' 挂载费用 - 平台收入' },
  { value: 'CoverMountingFee', label: ' 被挂载费用' },
  { value: 'CdnFee', label: ' CDN 费用' },
  { value: 'StorageFee', label: ' 仓储费用' },
  { value: 'TrafficFee', label: ' 流量费用' },
  { value: 'BuyGoods', label: ' 购买商品' },
]

// 发布任务类型和运行环境选项
const taskTypeOptions = [
  { value: 'job', label: '单次' },
  { value: 'Deployment', label: 'Docker' },
]
const runtimeOptions = [
  { value: 27, label: 'gromacs.2024.4' },
  { value: 25, label: 'docker-NB' },
]

// 显卡型号静态数据
const gpuList = [
  { name: 'NVIDIA RTX A4000', price: '0.97 元', mount: '0.2 元' },
  { name: 'NVIDIA GeForce GTX 1060', price: '0.2 元', mount: '0 元' },
  { name: 'NVIDIA GeForce GTX 1660', price: '0.25 元', mount: '0.1 元' },
  { name: 'NVIDIA GeForce GTX 1070', price: '0.25 元', mount: '0.1 元' },
  { name: 'NVIDIA GeForce RTX 2060 SUPER', price: '0.3 元', mount: '0.1 元' },
  { name: 'NVIDIA GeForce RTX 4070 Laptop GPU', price: '0.7 元', mount: '0.2 元' },
]
const gpuSearch = ref('')
const selectedGpus = ref<string[]>([])
const filteredGpus = computed(() => {
  if (!gpuSearch.value) return gpuList
  return gpuList.filter(g => g.name.toLowerCase().includes(gpuSearch.value.toLowerCase()))
})

async function submitData() {
  if (loading.value || cooldown.value > 0) return

  loading.value = true
  cooldown.value = 5

  // 5 秒倒计时
  const cooldownTimer = setInterval(() => {
    if (cooldown.value > 0) {
      cooldown.value--
    } else {
      clearInterval(cooldownTimer)
    }
  }, 1000)

  try {
    const userIds = userIdsRaw.value
      .split(',')
      .map(s => parseInt(s.trim(), 10))
      .filter(n => !isNaN(n))

    const res = await invoke<any>('slmv', {
      remark: remark.value,
      type: type.value,
      userIds,
      rmb: rmb.value,
    })

    const body = typeof res === 'string' ? res : JSON.stringify(res, null, 2)
    result.value = `✅ 成功：\n${body}`
    setTimeout(() => {
      result.value = ''
    }, 20000)
  } catch (e: any) {
    result.value = `❌ 失败：${e}`
    setTimeout(() => {
      result.value = ''
    }, 20000)
  } finally {
    loading.value = false
  }

}



function onRmbInput(event: Event) {
  const target = event.target as HTMLInputElement
  const value = target.value
  if (!/^\d*\.?\d*$/.test(value)) {
    target.value = rmb.value.toString() // 恢复之前的合法值
  }
}
function blockInvalidChars(event: KeyboardEvent) {
  const invalidChars = ['e', 'E', '+', '-']
  if (invalidChars.includes(event.key)) {
    event.preventDefault()
  }
}
function onUserIdInput(event: Event) {
  const target = event.target as HTMLTextAreaElement
  const sanitized = target.value.replace(/[^\d,]/g, '')
  target.value = sanitized
  userIdsRaw.value = sanitized
}
const loading = ref(false)
const cooldown = ref(0)


const showHistory = ref(false)

async function getmachins() {
  try {
    const ress = await invoke<any>('getmachinesitem', {
      machineId: machine_id.value
    })
    // 1. 先用正则提取 note 字段内容
    let noteStr = ''
    let match = String(ress).match(/note\": String\(\"(.*?)\"\)/)
    if (match && match[1]) {
      // 2. 反序列化转义字符
      noteStr = match[1].replace(/\\"/g, '"')
    }
    // 3. 解析 note 字段为对象
    let gpuNames: string[] = []
    let debugInfo = ''
    if (noteStr) {
      try {
        const noteObj: any = JSON.parse(noteStr)
        if (Array.isArray(noteObj.gpuModel)) {
          gpuNames = noteObj.gpuModel
            .map((g: any) => g.name)
            .filter((name: string) => name && name.includes('NVIDIA'))
        }
      } catch (e) {
        debugInfo = 'note 字段 JSON 解析异常：' + String(e)
      }
    } else {
      debugInfo = '未找到 note 字段'
    }
    if (gpuNames.length > 0) {
      machineResult.value = gpuNames.join('\n')
    } else {
      machineResult.value = debugInfo || '无 NVIDIA 显卡或数据异常'
    }
    return ress
  } catch (err) {
    machineResult.value = `❌ 查询失败：${err}`
    console.error('调用失败：', err)
    throw err
  }
}
const formattedResult = computed(() => {
  return result.value ? JSON.stringify(result.value, null, 2) : ''
})
const formattedMachineResult = computed(() => {
  return machineResult.value ? JSON.stringify(machineResult.value, null, 2) : ''
})

const showTaskPage = ref(false)
// 发布任务表单数据（与 Posttasks 结构体字段对应）
const taskForm = ref({
  name: '',
  desc: '',
  peer_income: '',
  expect_time: '',
  type: '',
  runtime: '',
  time_required: '',
  points: '',
  package: '',
  cpu_required: '',
  memory_required: '',
  disk_required: '',
  gpu_required: '',
  is_run_bore: false,
  domain_prefix: '',
  is_unrestricted_gpu: false,
  is_welfare: false,
  cuda_version: '',
  cuda_version_required: '',
  is_unrestricted_cuda_version: false,
  affinity: '',
  aversion: '',
  docker_compose_content: '',
  is_advanced_cuda: false,
  is_public_gpu: false,
})
const taskResult = ref('')
async function submitTask() {
  // 简单校验
  if (!taskForm.value.name || !taskForm.value.desc) {
    taskResult.value = '请填写任务名称和描述！'
    return
  }
  try {
    // 处理类型转换
    const payload = {
      name: taskForm.value.name,
      desc: taskForm.value.desc,
      peer_income: taskForm.value.peer_income ? parseFloat(taskForm.value.peer_income) : null,
      expect_time: taskForm.value.expect_time || null,
      type: taskForm.value.type,
      runtime: taskForm.value.runtime ? parseInt(taskForm.value.runtime) : 0,
      time_required: taskForm.value.time_required ? parseInt(taskForm.value.time_required) : null,
      points: taskForm.value.points ? parseInt(taskForm.value.points) : 0,
      package: taskForm.value.package ? taskForm.value.package : uuidv4(),
      cpu_required: taskForm.value.cpu_required ? parseInt(taskForm.value.cpu_required) : null,
      memory_required: taskForm.value.memory_required ? parseInt(taskForm.value.memory_required) : null,
      disk_required: taskForm.value.disk_required ? parseInt(taskForm.value.disk_required) : null,
      gpu_required: taskForm.value.gpu_required ? taskForm.value.gpu_required.split(',').map(x => parseInt(x.trim())).filter(x => !isNaN(x)) : [],
      is_run_bore: !!taskForm.value.is_run_bore,
      domain_prefix: taskForm.value.domain_prefix || null,
      is_unrestricted_gpu: !!taskForm.value.is_unrestricted_gpu,
      is_welfare: !!taskForm.value.is_welfare,
      cuda_version: taskForm.value.cuda_version || null,
      cuda_version_required: taskForm.value.cuda_version_required ? taskForm.value.cuda_version_required.split(',').map(x => x.trim()) : [],
      is_unrestricted_cuda_version: !!taskForm.value.is_unrestricted_cuda_version,
      affinity: taskForm.value.affinity ? taskForm.value.affinity.split(',').map(x => x.trim()) : [],
      aversion: taskForm.value.aversion ? taskForm.value.aversion.split(',').map(x => x.trim()) : [],
      docker_compose_content: taskForm.value.docker_compose_content || null,
      is_advanced_cuda: !!taskForm.value.is_advanced_cuda,
      is_public_gpu: !!taskForm.value.is_public_gpu,
    }
    const res = await invoke('posttasks', payload)
    taskResult.value = typeof res === 'string' ? res : JSON.stringify(res, null, 2)
  } catch (e) {
    taskResult.value = '发布失败：' + e
  }
}

</script>

<template>
  <div class="app-bg min-h-screen flex items-center justify-center py-8">
    <div class="card-container w-full max-w-3xl p-8 rounded-xl shadow-xl bg-white dark:bg-gray-900">
      <div class="task-header flex justify-between items-center mb-6">
        <h2 class="text-2xl font-bold">新建 Serverless 任务</h2>
        <button v-if="showTaskPage" class="submit-btn" @click="submitTask">提交任务</button>
      </div>
      <div class="flex mb-8 gap-2">
        <button :class="['tab-btn', !showTaskPage ? 'tab-active' : '']" @click="showTaskPage = false">资金操作</button>
        <button :class="['tab-btn', showTaskPage ? 'tab-active' : '']" @click="showTaskPage = true">发布任务</button>
      </div>
      <div v-if="!showTaskPage">
        <h2 class="text-3xl font-extrabold mb-8 text-center tracking-tight">资金操作</h2>
        <form @submit.prevent="submitData" class="space-y-6">
          <div>
            <label class="form-label">金额</label>
            <div class="relative">
              <input v-model.number="rmb" type="number" min="0" step="1" placeholder="输入人民币" class="form-input pr-12"
                @input="onRmbInput" @keydown="blockInvalidChars" />
              <span class="input-unit">元</span>
            </div>
          </div>
          <div>
            <label class="form-label">类型</label>
            <select v-model="type" class="form-input custom-select">
              <option v-for="opt in typeOptions" :key="opt.value" :value="opt.value">
                {{ opt.label }}
              </option>
            </select>
          </div>
          <div>
            <label class="form-label">用户 ID（逗号分隔）</label>
            <textarea v-model="userIdsRaw" placeholder="如：123,456" class="form-input h-20 resize-none"
              @input="onUserIdInput"></textarea>
          </div>
          <div>
            <label class="form-label">备注</label>
            <input v-model="remark" type="text" placeholder="备注" class="form-input" />
          </div>
          <button :disabled="loading || cooldown > 0" type="submit" class="main-btn">
            <span v-if="loading">⏳ 提交中...</span>
            <span v-else-if="cooldown">⏳ 冷却中 {{ cooldown }}s</span>
            <span v-else>🚀 提交操作</span>
          </button>
          <div v-if="result">
            <pre class="result-box">{{ result }}</pre>
          </div>
        </form>
        <hr class="my-8 border-gray-200 dark:border-gray-700" />
        <div>
          <label class="form-label">机器 ID</label>
          <div class="flex gap-2">
            <input v-model="machine_id" type="text" placeholder="输入机器 ID" class="form-input flex-1" />
            <button type="button" @click="getmachins" class="main-btn bg-green-500 hover:bg-green-600">查询</button>
          </div>
          <div v-if="machineResult">
            <pre class="result-box">{{ machineResult }}</pre>
          </div>
        </div>
      </div>
      <div v-else>
        <form class="task-form space-y-8">
          <!-- 基本信息 -->
          <fieldset class="section">
            <legend>基本信息</legend>
            <div class="form-row">
              <div class="form-col">
                <label>任务名<span class="required">*</span></label>
                <input v-model="taskForm.name" placeholder="例：测试任务" required class="form-input" />
              </div>
              <div class="form-col">
                <label>描述</label>
                <input v-model="taskForm.desc" placeholder="例：测试任务描述" class="form-input" />
              </div>
            </div>
            <div class="form-row">
              <div class="form-col">
                <label>任务类型<span class="required">*</span></label>
                <select v-model="taskForm.type" required class="form-input custom-select">
                  <option value="">请选择任务类型</option>
                  <option v-for="opt in taskTypeOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
              <div class="form-col">
                <label>运行环境<span class="required">*</span></label>
                <select v-model="taskForm.runtime" required class="form-input custom-select">
                  <option value="">请选择运行环境</option>
                  <option v-for="opt in runtimeOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
            </div>
            <div class="form-row">
              <div class="form-col">
                <label>任务文件<span class="required">*</span></label>
                <input v-model="taskForm.package" placeholder="选择文件夹" class="form-input" />
              </div>
            </div>
          </fieldset>
          <!-- 资源配置 -->
          <fieldset class="section">
            <legend>资源配置</legend>
            <div class="form-row">
              <div class="form-col">
                <label>单位收益<span class="required">*</span></label>
                <input v-model="taskForm.peer_income" placeholder="请输入单位收益" class="form-input" />
              </div>
              <div class="form-col">
                <label>预期单位时长</label>
                <input v-model="taskForm.expect_time" placeholder="请输入预期单位时长" class="form-input" />
              </div>
            </div>
            <div class="form-row">
              <div class="form-col">
                <label>所需节点数<span class="required">*</span></label>
                <input v-model="taskForm.points" placeholder="例：5" class="form-input" />
                <div class="form-desc">分为任务分配的节点（显卡）数量</div>
              </div>
            </div>
          </fieldset>
          <!-- CUDA 版本 -->
          <fieldset class="section">
            <legend>CUDA 版本<span class="required">*</span></legend>
            <div class="form-row">
              <div class="form-col">
                <div class="flex gap-4 mt-2 mb-2">
                  <label><input type="checkbox" v-model="taskForm.is_advanced_cuda"
                      :disabled="taskForm.is_unrestricted_cuda_version" /> 高级</label>
                  <label><input type="checkbox" v-model="taskForm.is_unrestricted_cuda_version" /> 不限制</label>
                </div>
                <template v-if="!taskForm.is_unrestricted_cuda_version">
                  <input v-model="taskForm.cuda_version" placeholder="如：11.2,12.2" class="form-input" />
                  <div class="form-desc">项目需用 CUDA 版本，多个用逗号分隔</div>
                </template>
              </div>
            </div>
          </fieldset>
          <!-- 显卡型号 -->
          <fieldset class="section">
            <legend>显卡型号</legend>
            <div class="flex gap-4 mt-2 mb-2">
              <label><input type="checkbox" v-model="taskForm.is_public_gpu" :disabled="taskForm.is_unrestricted_gpu" />
                全选</label>
              <label><input type="checkbox" v-model="taskForm.is_unrestricted_gpu" /> 不限制</label>
            </div>
            <template v-if="!taskForm.is_unrestricted_gpu">
              <div class="gpu-table">
                <input placeholder="搜索显卡型号" v-model="gpuSearch" class="gpu-search form-input mb-2" />
                <table class="w-full text-sm">
                  <thead>
                    <tr>
                      <th></th>
                      <th>型号</th>
                      <th>单价/小时</th>
                      <th>挂载费/小时</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="gpu in filteredGpus" :key="gpu.name">
                      <td><input type="checkbox" v-model="selectedGpus" :value="gpu.name" /></td>
                      <td>{{ gpu.name }}</td>
                      <td>{{ gpu.price }}</td>
                      <td>{{ gpu.mount }}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </template>
          </fieldset>
          <div class="welfare-switch">
            <label>是否为福利任务<span class="required">*</span></label>
            <input type="checkbox" v-model="taskForm.is_welfare" class="welfare-checkbox" />
          </div>
        </form>
        <div v-if="taskResult" class="result-box mt-4">{{ taskResult }}</div>
      </div>
    </div>
  </div>
</template>


<style scoped>
.app-bg {
  background: #f6f8fa;
}

.card-container {
  box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.12);
}

.form-label {
  display: block;
  margin-bottom: 0.25rem;
  font-size: 0.98rem;
  color: #6b7280;
  font-weight: 500;
  letter-spacing: 0.01em;
}

.form-input {
  width: 100%;
  padding: 0.85rem 1rem;
  font-size: 1.05rem;
  border: 1.5px solid #e5e7eb;
  border-radius: 0.7rem;
  background-color: #f9fafb;
  color: #222;
  transition: border-color 0.2s, box-shadow 0.2s;
  box-sizing: border-box;
  outline: none;
}

.form-input:focus {
  border-color: #2563eb;
  background: #fff;
  box-shadow: 0 0 0 2px #3b82f633;
}

.input-unit {
  position: absolute;
  right: 1rem;
  top: 50%;
  transform: translateY(-50%);
  color: #b0b3b8;
  font-size: 1rem;
  pointer-events: none;
}

.custom-select {
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 140 140' width='12' height='12' xmlns='http://www.w3.org/2000/svg'%3E%3Cpolyline points='20,50 70,100 120,50' stroke='%23999' stroke-width='20' fill='none' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 1rem center;
  background-size: 1em;
}

.main-btn {
  display: block;
  min-width: 120px;
  max-width: 260px;
  margin: 0.5rem auto 0 auto;
  padding: 0.55rem 1.2rem;
  font-size: 1rem;
  font-weight: 600;
  color: #fff;
  background: linear-gradient(90deg, #3b82f6 0%, #2563eb 100%);
  border: none;
  border-radius: 0.5rem;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.10);
  transition: background 0.2s, transform 0.1s;
  cursor: pointer;
}

.main-btn:hover:not(:disabled) {
  background: linear-gradient(90deg, #2563eb 0%, #1e40af 100%);
  transform: scale(1.04);
}

.main-btn:disabled {
  background: #b6c3d1;
  cursor: not-allowed;
  box-shadow: none;
}

.result-box {
  background-color: #f3f4f6;
  color: #374151;
  border-radius: 0.7rem;
  padding: 1rem;
  margin-top: 0.5rem;
  font-size: 0.97rem;
  white-space: pre-wrap;
  word-break: break-all;
}

.dark .result-box {
  background-color: #1e293b;
  color: #f3f4f6;
}

@media (max-width: 640px) {
  .card-container {
    padding: 1.2rem;
  }

  .form-input {
    padding: 0.7rem 0.8rem;
    font-size: 0.98rem;
  }

  .main-btn {
    font-size: 0.98rem;
    padding: 0.7rem 1rem;
  }
}

.tab-btn {
  flex: 1;
  padding: 0.7rem 80px;
  font-size: 1.1rem;
  font-weight: 600;
  border: none;
  border-radius: 0.7rem 0.7rem 0 0;
  background: #e5e7eb;
  color: #374151;
  transition: background 0.2s, color 0.2s;
  cursor: pointer;
}

.tab-btn.tab-active {
  background: linear-gradient(90deg, #3b82f6 0%, #2563eb 100%);
  color: #fff;
}

.section {
  border: 1px solid #e5e7eb;
  border-radius: 1rem;
  padding: 1.5rem 1rem 1rem 1rem;
  margin-bottom: 1.5rem;
}

legend {
  font-weight: bold;
  font-size: 1.1rem;
  margin-bottom: 0.5rem;
}

.form-row {
  display: flex;
  gap: 1.5rem;
  margin-bottom: 1rem;
}

.form-col {
  flex: 1;
  min-width: 180px;
}

.required {
  color: #e53e3e;
  margin-left: 0.2em;
}

.form-desc {
  color: #888;
  font-size: 0.92em;
  margin-top: 0.2em;
}

.task-header {
  margin-bottom: 1.5rem;
}

.submit-btn {
  background: #10b981;
  color: #fff;
  font-weight: 600;
  border: none;
  border-radius: 0.5rem;
  padding: 0.6rem 1.5rem;
  font-size: 1rem;
  cursor: pointer;
  transition: background 0.2s;
}

.submit-btn:hover {
  background: #059669;
}

.gpu-table {
  margin-top: 0.5rem;
}

.gpu-search {
  width: 220px;
}

.welfare-switch {
  margin-top: 1.5rem;
  text-align: center;
}

.welfare-checkbox {
  margin-left: 0.5rem;
}
</style>
