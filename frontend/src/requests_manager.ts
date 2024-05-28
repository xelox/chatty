import type { Notification } from "./stores/inbox";
import notification_manager from "./notification_manager"

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

  /** 
   * @param path starting with /
   * @param options can be used to add behaviour.
   * */
  public get = (path: string, options?: RequestOptions) => { this.request(path, "GET", undefined, options); }

  /** 
   * @param path starting with /
   * @param item to be serialized to json and sent to server.
   * @param options can be used to add behaviour.
   * */
  public post = (path: string, item: any, options?: RequestOptions) => { this.request(path, "POST", JSON.stringify(item), options); }

  /** 
   * @param path starting with /
   * @param form to be serialized to json and sent to server.
   * @param options can be used to add behaviour.
   * */
  public post_form = (path: string, form: FormData, options?: RequestOptions) => { this.request(path, "POST", form, options, true); }

  /** 
   * @param path starting with /
   * @param item to be serialized to json and sent to server.
   * @param options can be used to add behaviour.
   * */
  public put = (path: string, item: any, options?: RequestOptions) => { this.request(path, "PUT", JSON.stringify(item), options); }

  /** 
   * @param path starting with /
   * @param item to be serialized to json and sent to server.
   * @param options can be used to add behaviour.
   * */
  public delete = (path: string, item: any, options?: RequestOptions) => { this.request(path, "DELETE", JSON.stringify(item), options); }

  /** 
   * @param path starting with /
   * @param item to be serialized to json and sent to server.
   * @param options can be used to add behaviour.
   * */
  public patch = (path: string, item: any, options?: RequestOptions) => { this.request(path, "PATCH", JSON.stringify(item), options); }
}

export const requests_manager = new RequestsManager("http://localhost:8080");
