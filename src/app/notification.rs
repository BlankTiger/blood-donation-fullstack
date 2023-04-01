use leptos::*;

pub enum NotificationType {
    Info,
    Error,
}

#[component]
pub fn Notification(cx: Scope, msg: String, notification_type: NotificationType) -> impl IntoView {
    match notification_type {
        NotificationType::Info => view! { cx,
                                      <div
                                          id="alert-1"
                                          class="flex p-4 mb-4 text-blue-800 rounded-lg bg-blue-200"
                                          role="alert"
                                      >
                                          <svg
                                              aria-hidden="true"
                                              class="flex-shrink-0 w-5 h-5"
                                              fill="currentColor"
                                              viewBox="0 0 20 20"
                                              xmlns="http://www.w3.org/2000/svg"
                                          >
                                              <path
                                                  fill-rule="evenodd"
                                                  d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                                                  clip-rule="evenodd"
                                              ></path>
                                          </svg>
                                          <span class="sr-only">"Info"</span>
                                          <div class="ml-3 text-sm font-medium">{msg}</div>
                                      </div>
                                  }.into_view(cx),
        NotificationType::Error => view! { cx,
                                       <div
                                           id="alert-2"
                                           class="flex p-4 mb-4 text-red-800 rounded-lg bg-red-200"
                                           role="alert"
                                       >
                                           <svg
                                               aria-hidden="true"
                                               class="flex-shrink-0 w-5 h-5"
                                               fill="currentColor"
                                               viewBox="0 0 20 20"
                                               xmlns="http://www.w3.org/2000/svg"
                                           >
                                               <path
                                                   fill-rule="evenodd"
                                                   d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                                                   clip-rule="evenodd"
                                               ></path>
                                           </svg>
                                           <span class="sr-only">"Info"</span>
                                           <div class="ml-3 text-sm font-medium">{msg}</div>
                                       </div>
                                   }.into_view(cx)
    }
}
