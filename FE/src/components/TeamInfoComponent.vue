<template>
  <v-row>
    <!-- Render each member in a responsive card grid -->
    <v-col v-for="member in members.members" :key="member.id" cols="12" sm="6" md="4" lg="3">
      <v-card elevation="2" class="team-member-card">
        <v-card-item>
          <v-img :src="member.avatar_url" aspect-ratio="1" class="avatar" />
        </v-card-item>

        <v-card-item>
          <div class="member-info">
            <h3>{{ member.name || member.login }}</h3>
            <p class="username">
              {{ member.login }}
            </p>
            <p v-if="member.email" class="email">
              {{ member.email }}
            </p>
            <v-btn small text :href="member.html_url" target="_blank"> GitHub Profile </v-btn>
          </div>
        </v-card-item>

        <v-card-actions class="actions">
          <v-btn icon :href="member.repos_url" target="_blank" title="Repositories">
            <v-icon>mdi-source-repository</v-icon>
          </v-btn>
          <v-btn icon :href="member.followers_url" target="_blank" title="Followers">
            <v-icon>mdi-account-multiple</v-icon>
          </v-btn>
          <v-btn icon :href="member.gists_url" target="_blank" title="Gists">
            <v-icon>mdi-code-tags</v-icon>
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import type { PropType } from 'vue'
import { defineComponent } from 'vue'
import type { Members } from '@/model/Members'

export default defineComponent({
  name: 'TeamMembersComponent',
  props: {
    members: {
      type: Object as PropType<Members>,
      required: true
    }
  }
})
</script>

<style scoped>
.team-member-card {
  margin-bottom: 20px;
  text-align: center;
}

.avatar {
  border-radius: 50%;
  width: 100px;
  height: 100px;
  margin: 0 auto;
}

.member-info h3 {
  font-size: 1.2em;
  margin: 10px 0 5px;
}

.member-info .username {
  font-size: 0.9em;
  color: #555;
}

.member-info .email {
  font-size: 0.9em;
  color: #777;
}

.actions {
  justify-content: center;
}
</style>
