export class Team {
  id: number;
  node_id: string;
  name: string;
  slug: string;
  description: string | null;
  privacy: string;
  notification_setting: string;
  permission: string;
  // permissions: TeamPermissions;
  url: string;
  html_url: string;
  members_url: string;
  repositories_url: string;
  parent: TeamSimple | null;

  constructor(data: any) {
    this.id = data.id;
    this.node_id = data.node_id;
    this.name = data.name;
    this.slug = data.slug;
    this.description = data.description;
    this.privacy = data.privacy;
    this.notification_setting = data.notification_setting;
    this.permission = data.permission;
    // this.permissions = new TeamPermissions(data.permissions);
    this.url = data.url;
    this.html_url = data.html_url;
    this.members_url = data.members_url;
    this.repositories_url = data.repositories_url;
    this.parent = data.parent ? new TeamSimple(data.parent) : null;
  }
}

// export class TeamPermissions {
//   pull: boolean;
//   triage: boolean;
//   push: boolean;
//   maintain: boolean;
//   admin: boolean;

//   constructor(data: any) {
//     // this.pull = data.pull || null;
//     this.triage = data.triage;
//     this.push = data.push;
//     this.maintain = data.maintain;
//     this.admin = data.admin;
//   }
// }

export class TeamSimple {
  id: number;
  node_id: string;
  url: string;
  members_url: string;
  name: string;
  description: string | null;
  permission: string;
  privacy: string;
  notification_setting: string;
  html_url: string;
  repositories_url: string;
  slug: string;

  constructor(data: any) {
    this.id = data.id;
    this.node_id = data.node_id;
    this.url = data.url;
    this.members_url = data.members_url;
    this.name = data.name;
    this.description = data.description;
    this.permission = data.permission;
    this.privacy = data.privacy;
    this.notification_setting = data.notification_setting;
    this.html_url = data.html_url;
    this.repositories_url = data.repositories_url;
    this.slug = data.slug;
  }
}