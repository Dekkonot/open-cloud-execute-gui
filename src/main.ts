import { invoke } from "@tauri-apps/api/core";

import { display_return, script_editor_content } from "./function.js";

const script_timeout = document.getElementById(
    "script-timeout"
)! as HTMLInputElement;

const place_id = document.getElementById("place-id")! as HTMLInputElement;
const universe_id = document.getElementById("universe-id")! as HTMLInputElement;
const place_version = document.getElementById(
    "place-version"
)! as HTMLInputElement;
const api_key = document.getElementById("api-key")! as HTMLInputElement;

const session_id = document.getElementById("session-id")! as HTMLInputElement;
const task_id = document.getElementById("task-id")! as HTMLInputElement;

const upload_script = document.getElementById(
    "upload-script"
)! as HTMLButtonElement;

const output = document.getElementById("output")! as HTMLDivElement;
const return_values = document.getElementById(
    "return-values"
)! as HTMLDivElement;

type OpenCloudExecutionTask = {
    path: string;
    user: string;
    state: OpenCloudState;
    script: string;
    createTime?: string;
    updateTime?: string;
    output?: {
        results: string[];
    };
    error?: {
        code: OpenCloudError;
        message: string;
    };
};

type StructuredMessage = {
    message: string;
    createTime: string;
    messageType: MessageType;
};

type OpenCloudError =
    | "ERROR_CODE_UNSPECIFIED"
    | "SCRIPT_ERROR"
    | "DEADLINE_EXCEEDED"
    | "OUTPuT_SIZE_LIMIT_EXCEEDED"
    | "INTERNAL_ERROR";

type OpenCloudState =
    | "STATE_UNSPECIFIED"
    | "QUEUED"
    | "PROCESSING"
    | "CANCELLED"
    | "COMPLETE"
    | "FAILED";

type MessageType = "INFO" | "OUTPUT" | "WARNING" | "ERROR";

function emit_line(line_type: MessageType, message: string) {
    const element = document.createElement("span");
    element.className = `output-${line_type}`;
    element.textContent = message;
    output.appendChild(element);
}

function clear_output() {
    output.replaceChildren();
}

function clear_results() {
    return_values.replaceChildren();
}

function lock_fields() {
    universe_id.readOnly = true;
    place_id.readOnly = true;
    place_version.readOnly = true;
    api_key.readOnly = true;
}

function unlock_fields() {
    universe_id.readOnly = false;
    place_id.readOnly = false;
    place_version.readOnly = false;
    api_key.readOnly = false;
}

async function create_task(): Promise<OpenCloudExecutionTask> {
    const version_number = place_version.value;
    const timeout = script_timeout.value;

    const url = await invoke("create_task_url", {
        placeId: place_id.value,
        universeId: universe_id.value,
        versionNumber: version_number === "" ? null : version_number,
    });
    emit_line("INFO", "Uploading Luau execution task...");

    return invoke("create_task", {
        apiKey: api_key.value,
        taskUrl: url,
        script: script_editor_content(),
        timeout: timeout === "" ? null : parseFloat(timeout),
    });
}

async function await_task(
    task: OpenCloudExecutionTask
): Promise<OpenCloudExecutionTask> {
    emit_line("INFO", "Waiting for the task to finish...");
    return invoke("await_task", {
        apiKey: api_key.value,
        path: task.path,
    });
}

// async function get_logs_flat(task: OpenCloudExecutionTask): Promise<string[]> {
//     return invoke("get_logs_flat", {
//         apiKey: api_key.value,
//         path: task.path,
//     });
// }

async function get_logs_structured(
    task: OpenCloudExecutionTask
): Promise<StructuredMessage[]> {
    return invoke("get_logs_structured", {
        apiKey: api_key.value,
        path: task.path,
    });
}

type OpenCloudPath = {
    universe_id: string;
    place_id: string;
    place_version: string;
    session_id: string;
    task_id: string;
};

const task_regex =
    /universes\/(\d+)\/places\/(\d+)\/versions\/(\d+)\/luau-execution-sessions\/([\w\-]+)\/tasks\/([\w\-]+)/;
function parse_task(path: string): OpenCloudPath | null {
    const captures = task_regex.exec(path);
    if (captures) {
        return {
            universe_id: captures[1],
            place_id: captures[2],
            place_version: captures[3],
            session_id: captures[4],
            task_id: captures[5],
        };
    }
    return null;
}

window.addEventListener("DOMContentLoaded", () => {
    upload_script.onclick = async () => {
        clear_results();
        clear_output();
        let task = await create_task().catch((e) => {
            window.alert(e);
        });
        if (task) {
            lock_fields();
            let path = parse_task(task.path);
            if (path) {
                universe_id.value = path.universe_id;
                place_id.value = path.place_id;
                place_version.value = path.place_version;
                session_id.value = path.session_id;
                task_id.value = path.task_id;
            } else {
                window.alert(
                    "Cannot parse URL of task. This will only impact the display of the IDs, but please report it!"
                );
            }
            const task_result = await await_task(task).catch((e) => {
                window.alert(e);
            });
            if (task_result) {
                if (task_result.error) {
                    window.alert(
                        `Script failed to finish: ${task_result.error.code}\n${task_result.error.message}`
                    );
                }
                const output = await get_logs_structured(task);

                for (const line of output) {
                    emit_line(line.messageType, line.message);
                }

                display_return(task_result.output?.results);

                unlock_fields();
            }
        }
    };
});
