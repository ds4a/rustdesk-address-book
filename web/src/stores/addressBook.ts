import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as abApi from '../api/addressBook'

export const useAddressBookStore = defineStore('addressBook', () => {
  const abGuid = ref('')
  const peers = ref<abApi.Peer[]>([])
  const tags = ref<abApi.Tag[]>([])
  const totalPeers = ref(0)
  const selectedTag = ref<string | null>(null)
  const loading = ref(false)

  async function init() {
    const res = await abApi.getPersonalAb()
    abGuid.value = res.data.guid
    await Promise.all([loadPeers(), loadTags()])
  }

  async function loadPeers() {
    if (!abGuid.value) return
    loading.value = true
    try {
      const res = await abApi.getPeers(abGuid.value)
      peers.value = res.data
      totalPeers.value = res.total
    } finally {
      loading.value = false
    }
  }

  async function loadTags() {
    if (!abGuid.value) return
    const res = await abApi.getTags(abGuid.value)
    tags.value = res.data
  }

  async function addPeer(peer: Partial<abApi.Peer>) {
    await abApi.addPeer(abGuid.value, peer)
    await loadPeers()
  }

  async function removePeer(ids: string[]) {
    await abApi.deletePeer(abGuid.value, ids)
    await loadPeers()
  }

  async function addTag(name: string, color: number) {
    await abApi.addTag(abGuid.value, name, color)
    await loadTags()
  }

  async function removeTag(names: string[]) {
    await abApi.deleteTag(abGuid.value, names)
    await loadTags()
  }

  const filteredPeers = () => {
    if (!selectedTag.value) return peers.value
    return peers.value.filter(p => p.tags.includes(selectedTag.value!))
  }

  return {
    abGuid, peers, tags, totalPeers, selectedTag, loading,
    init, loadPeers, loadTags, addPeer, removePeer, addTag, removeTag,
    filteredPeers,
  }
})
