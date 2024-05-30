import type { Notification } from "./stores/inbox";
import notification_manager from "./notification_manager"
import { type SchemaInitData, type SchemaPeer, type SchemaPeerList, type SchemaUserInfo } from "./stores/data";
import type { SchemaChannel, SchemaMessage, SchemaUpMessage } from "./stores/messages";

/** 
 * @param exectly validation fails if result does not match this value exactly.
 * @param validate has return true for ok results, false otherwise.
 * @param notify_fail if true it will a notify the user.
 * @param fail_action callback on fail.
 * @param succeed_action callback on succeed.
 * */
export type RequestOptions = {
  exactly?: any,
  validate?: (result: any) => boolean,
  notify_fail?: boolean,
  fail_action?: (result?: any) => void,
  succeed_action?: (result?: any) => void,
}

class RequestsManager {
  private base = "http://localhost:8080"
  constructor(base: string){
    this.base = base;
  }
  private handle_error = (err: any, options?: RequestOptions) => {
    console.error(err);
    err = (err instanceof String || typeof err === 'string') ? err : err.message;
    err = (err instanceof String || typeof err === 'string') ? err : err.error;
    if (!(err instanceof String || typeof err === 'string')) {
      // WARNING the heck is this?
      return;
    }

    const err_message = err as string;
    if(options?.notify_fail) {
      let n: Notification = {
        ts: Number(new Date),
        content: err_message,
        source: "System",
      }
      notification_manager.notify(n, 'system');
    }
    if (options?.fail_action) options.fail_action(err_message);
  }

  private request = (path: string, method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH", body?: string | FormData, options?: RequestOptions, is_form?: boolean) => {
    const headers: HeadersInit = [];
    if (!is_form) headers.push(["Content-Type", "application/json"] );

    fetch(`${this.base}${path}`, {
      method,
      headers,
      body
    }).then(res=>{
        if (!res.ok) {
          return this.handle_error(`${path}: ${res.statusText}`, options);
        }
        res.text().then(res=>{
          if (!options) return;
          if (options.exactly) {
            if (options.exactly !== res) return this.handle_error(res, options);
          }
          if (options.validate) {
            if (!options.validate(res)) return this.handle_error(res, options);
          }
          if (options.succeed_action) options.succeed_action(res);
        }).catch(err => this.handle_error(err, options))
      }).catch(err => this.handle_error(err, options))
  }

  public patch_profile(form: FormData) {
    return new Promise<SchemaUserInfo>((res, rej) => {
      const opts: RequestOptions = {
        notify_fail: true,
        succeed_action: (data) => { res(JSON.parse(data)); },
        fail_action: rej
      }
      this.request('/api/profile', "PATCH", form, opts, true);
    })
  }

  public signup(username: string, password: string) {
    return new Promise<void>((res, rej) => {
      const opts: RequestOptions = {
        notify_fail: true,
        succeed_action: res,
        fail_action: rej,
      }
      this.request('/api/singup', 'POST', JSON.stringify({username, password}), opts);
    })
  }

  public signin(username: string, password: string) {
    return new Promise<void>((res, rej) => {
      const opts: RequestOptions = {
        notify_fail: true,
        succeed_action: res,
        fail_action: rej,
      }
      this.request('/api/signin', 'POST', JSON.stringify({username, password}), opts);
    })
  }

  public logout() {
    return new Promise<void>((res, rej) => {
      const opts: RequestOptions = {
        notify_fail: true,
        succeed_action: res,
        fail_action: rej
      }
      this.request('/api/logout', 'GET', undefined, opts);
    })
  }

  public get_channel_info(channel_id: string) {
    return new Promise<SchemaChannel>((res, rej) => {
      const opts: RequestOptions = {
        succeed_action: (data) => { res(JSON.parse(data) as SchemaChannel) },
        fail_action: rej
      }
      this.request(`/api/channel/${channel_id}`, 'GET', undefined, opts);
    })
  }

  public load_messages(channel_id: string, ts: number) {
    return new Promise<SchemaMessage[]>((res, rej) => {
      const opts: RequestOptions = {
        succeed_action: (messages) => { res(JSON.parse(messages) as SchemaMessage[]); },
        fail_action: rej
      }
      this.request(`/api/messages/${channel_id}/${ts}`, 'GET', undefined, opts);
    })
  }

  public send_message(message: SchemaUpMessage) {
    return new Promise<void>((res, rej) => {
      const opts: RequestOptions = {
        notify_fail: true,
        succeed_action: res,
        fail_action: rej,
      }
      this.request('/api/message', 'POST', JSON.stringify(message), opts);
    })
  }

  public delete_message(message: SchemaMessage) {
    return new Promise<void>((res, rej) => {
      const opts: RequestOptions = {
        notify_fail: true,
        succeed_action: res,
        fail_action: rej,
      }
      this.request('/api/message', 'DELETE', JSON.stringify(message), opts);
    })
  }


  public patch_relation(action: "cancel" | "accept" | "refuse", relation_id: string) {
    return new Promise<void>((res, rej) => {
      const opts: RequestOptions = {
        notify_fail: true,
        succeed_action: res,
        fail_action: rej
      }
      this.request('/api/relation', 'PATCH', JSON.stringify({relation_id, action}), opts);
    });
  }

  public init_client() {
    type Relation = {
      id: string, a: string, b: string, sender: string, accepted: true, created_at: number, accepted_at: number,
    };

    type DataResponse = {
      relations: {user: SchemaUserInfo, relation: Relation}[]
      user_info: SchemaUserInfo,
    }

    return new Promise<SchemaInitData>((res, rej) => {
      const opts: RequestOptions = {
        notify_fail: true,
        fail_action: rej,
        succeed_action: (json) => {
          const data = JSON.parse(json) as DataResponse;

          const friends: SchemaPeerList = {}
          const pending_out: SchemaPeerList = {}
          const pending_in: SchemaPeerList = {}
          const known_users: SchemaPeer[] = [data.user_info];

          for (const item of data.relations) {
            known_users.push(item.user);
            if (item.relation.accepted) {
              friends[item.relation.id] = {...item.user, relation_id: item.relation.id}
            } else if (item.relation.sender === data.user_info.id) {
              pending_out[item.relation.id] = {...item.user, relation_id: item.relation.id}
            } else {
              pending_in[item.relation.id] = {...item.user, relation_id: item.relation.id}
            }
          }

          res({
            user: data.user_info,
            friends,
            pending_in,
            pending_out,
            known_users
          });
        },
      }
      this.request('/api/init', 'GET', undefined, opts);
    })
  }
}

export const requests_manager = new RequestsManager("http://localhost:8080");
