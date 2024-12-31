### Event and Command  
- The major differences between a Tauri command and a Tauri event are that events have no strong type support, event payloads are always JSON strings making them not suitable for bigger messages.  

### Event Traits  
- The AppHandle and WebviewWindow types implement the event system traits `Listener` and `Emitter`.  

#### Global Events  
- Global events are delivered to all listeners  
```rust
use tauri::{AppHandle, Emitter};
#[tauri::command]
fn download(app: AppHandle, url: String) {
  app.emit("download-started", &url).unwrap();
  for progress in [1, 15, 50, 80, 100] {
    app.emit("download-progress", progress).unwrap();
  }
  app.emit("download-finished", &url).unwrap();
}
```

#### Webview Events  
- To trigger an event to a listener registered by a specific webview you can use the emit_to function  
- Webview-specific events are not triggered to regular global event listeners. To listen to any event you must use the listen_any function instead of listen.  
```rust
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn login(app: AppHandle, user: String, password: String) {
  let authenticated = user == "tauri-apps" && password == "tauri";
  let result = if authenticated { "loggedIn" } else { "invalidCredentials" };
  app.emit_to("login", "login-result", result).unwrap();
}
```

#### Event Payload  
- The event payload can be any serializable type that also implements Clone  

### Listening on Frontend  

#### Listen global event  
```ts
import { listen } from '@tauri-apps/api/event';

type DownloadStarted = {
  url: string;
  downloadId: number;
  contentLength: number;
};

listen<DownloadStarted>('download-started', (event) => {
  console.log(
    `downloading ${event.payload.contentLength} bytes from ${event.payload.url}`
  );
});
```

#### Listening to webview-specific events  
```ts
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const appWebview = getCurrentWebviewWindow();
appWebview.listen<string>('logged-in', (event) => {
  localStorage.setItem('session-token', event.payload);
});
```

#### Stop listening  
- The listen function keeps the event listener registered for the entire lifetime of the application.  
- Always use the unlisten function when your execution context goes out of scope such as when a component is unmounted.  
- When the page is reloaded or you navigate to another URL the listeners are unregistered automatically. This does not apply to a Single Page Application (SPA) router though.  
```ts
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('download-started', (event) => {});
unlisten();
```

### Listening on Backend  



