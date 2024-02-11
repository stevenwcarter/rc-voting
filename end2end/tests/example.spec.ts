import { test, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
  await page.goto("http://localhost:3232/");

  await expect(page).toHaveTitle("Ranked Choice Voting");

  await expect(page.locator("h1")).toHaveText("Log in to RC Voting");
});
