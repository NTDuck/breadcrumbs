import { app } from "$lib";
import type { PageLoad } from "$types";

// SPA
// export const ssr: boolean = false;
// export const prerender: boolean = false;

export const load: PageLoad = async () => {
  return {
    tasksPaginationResponse: await app.viewTasks({
      paginationRequest: {
        pageNumber: 1,
        maxPageSize: 44,
      },
    }),
  };
};

export const actions = {
  create: async ({ request }) => {
    const formData = await request.formData();
    await app.createTask({
      taskDescription: formData.get("task-description"),
    });
  },
}
