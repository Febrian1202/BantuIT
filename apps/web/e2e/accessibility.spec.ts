import { test, expect } from "@playwright/test";

test.describe("WCAG 2.2 AA Accessibility & Keyboard Navigation E2E", () => {
  test("1. Document HTML semantics: lang='id' and unique title", async ({ page }) => {
    await page.goto("/login");

    const htmlLang = await page.locator("html").getAttribute("lang");
    expect(htmlLang).toBe("id");

    const title = await page.title();
    expect(title).toContain("BantuIT");
  });

  test("2. Keyboard tab navigation on login page", async ({ page }) => {
    await page.goto("/login");

    // Focus starts or moves via Tab
    const emailInput = page.locator('input[name="email"]');
    const passwordInput = page.locator('input[name="password"]');
    const submitBtn = page.locator('button[type="submit"]');

    await emailInput.focus();
    await expect(emailInput).toBeFocused();

    // Tab to password
    await page.keyboard.press("Tab");
    await expect(passwordInput).toBeFocused();

    // Tab to toggle password visibility button
    await page.keyboard.press("Tab");
    const togglePasswordBtn = page.getByRole("button", {
      name: /tampilkan kata sandi|sembunyikan kata sandi/i,
    });
    await expect(togglePasswordBtn).toBeFocused();

    // Tab to submit button
    await page.keyboard.press("Tab");
    await expect(submitBtn).toBeFocused();
  });

  test("3. Modal dialog dismisses on Escape key press", async ({ page, context }) => {
    await context.clearCookies();
    await page.goto("/login");
    await page.fill('input[name="email"]', "admin@bantuit.test");
    await page.fill('input[name="password"]', "Password123!");
    await page.click('button[type="submit"]');
    await page.waitForURL((url) => !url.pathname.includes("/login"), {
      timeout: 15000,
    });

    // Go to admin users where a Dialog is available
    await page.goto("/admin/users");
    const tambahUserBtn = page.getByRole("button", { name: "Tambah Pengguna" });
    await expect(tambahUserBtn).toBeVisible({ timeout: 10000 });
    await tambahUserBtn.click();

    // Dialog should be open
    const dialog = page.locator('[role="dialog"]');
    await expect(dialog).toBeVisible({ timeout: 5000 });

    // Pressing Escape closes dialog
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden({ timeout: 5000 });
  });

  test("4. Tables enforce scope='col' on table header cells", async ({ page, context }) => {
    await context.clearCookies();
    await page.goto("/login");
    await page.fill('input[name="email"]', "admin@bantuit.test");
    await page.fill('input[name="password"]', "Password123!");
    await page.click('button[type="submit"]');
    await page.waitForURL((url) => !url.pathname.includes("/login"), {
      timeout: 15000,
    });

    await page.goto("/admin/users");
    await expect(page.locator("table")).toBeVisible({ timeout: 10000 });

    // Every <th> in the table should have scope="col"
    const ths = page.locator("table th");
    const count = await ths.count();
    expect(count).toBeGreaterThan(0);

    for (let i = 0; i < count; i++) {
      const scope = await ths.nth(i).getAttribute("scope");
      expect(scope).toBe("col");
    }
  });

  test("5. Form validation errors communicate aria-invalid", async ({ page }) => {
    await page.goto("/login");

    // Submit without filling form
    await page.click('button[type="submit"]');

    const emailInput = page.locator('input[name="email"]');
    await expect(emailInput).toHaveAttribute("aria-invalid", "true");
  });
});
