export class SimpleUser {
  name: string | null;
  email: string | null;
  login: string;
  id: number;
  node_id: string;
  avatar_url: string;
  gravatar_id: string | null;
  url: string;
  html_url: string;
  followers_url: string;
  following_url: string;
  gists_url: string;
  starred_url: string;
  subscriptions_url: string;
  organizations_url: string;
  repos_url: string;
  events_url: string;
  received_events_url: string;
  type: string;
  site_admin: boolean;
  starred_at: string | undefined;

  constructor(data: any) {
    this.name = data.name;
    this.email = data.email;
    this.login = data.login;
    this.id = data.id;
    this.node_id = data.node_id;
    this.avatar_url = data.avatar_url;
    this.gravatar_id = data.gravatar_id;
    this.url = data.url;
    this.html_url = data.html_url;
    this.followers_url = data.followers_url;
    this.following_url = data.following_url;
    this.gists_url = data.gists_url;
    this.starred_url = data.starred_url;
    this.subscriptions_url = data.subscriptions_url;
    this.organizations_url = data.organizations_url;
    this.repos_url = data.repos_url;
    this.events_url = data.events_url;
    this.received_events_url = data.received_events_url;
    this.type = data.type;
    this.site_admin = data.site_admin;
    this.starred_at = data.starred_at;
  }
}

export class Members {
  members: SimpleUser[];

  constructor(data: any) {
    if (Array.isArray(data)) {
      this.members = data.map((item: any) => new SimpleUser(item));
    } else if (typeof data === 'object' && data !== null) {
      this.members = [new SimpleUser(data)];
    } else {
      console.error('Error: Invalid data type', data);
      this.members = [];
    }
  }
}