import { screen, render, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import {
  DemoAccountCards,
  DEMO_ACCOUNTS,
} from "@/components/auth/demo-account-cards";

describe("DemoAccountCards Component", () => {
  it("renders all 4 demo role cards with correct labels and emails", () => {
    const onSelectMock = vi.fn();
    render(<DemoAccountCards onSelect={onSelectMock} />);

    for (const acc of DEMO_ACCOUNTS) {
      expect(
        screen.getByRole("button", { name: new RegExp(acc.role, "i") })
      ).toBeInTheDocument();
      expect(screen.getByText(acc.email)).toBeInTheDocument();
      expect(screen.getByText(new RegExp(acc.label, "i"))).toBeInTheDocument();
    }
  });

  it("calls onSelect with the selected email when a card is clicked", () => {
    const onSelectMock = vi.fn();
    render(<DemoAccountCards onSelect={onSelectMock} />);

    const adminBtn = screen.getByRole("button", { name: /administrator/i });
    fireEvent.click(adminBtn);

    expect(onSelectMock).toHaveBeenCalledTimes(1);
    expect(onSelectMock).toHaveBeenCalledWith("admin@bantuit.test");
  });

  it("disables all buttons when disabled prop is true", () => {
    const onSelectMock = vi.fn();
    render(<DemoAccountCards onSelect={onSelectMock} disabled={true} />);

    const buttons = screen.getAllByRole("button");
    for (const btn of buttons) {
      expect(btn).toBeDisabled();
    }

    fireEvent.click(buttons[0]);
    expect(onSelectMock).not.toHaveBeenCalled();
  });

  it("does not render any icons or svg elements in the demo cards", () => {
    const onSelectMock = vi.fn();
    const { container } = render(<DemoAccountCards onSelect={onSelectMock} />);
    expect(container.querySelector("svg")).toBeNull();
  });
});
