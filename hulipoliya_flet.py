import flet as ft

# ─── Цвета ─────────────────────────────────
BASE_COLOR = ft.Colors.BLUE_GREY_400
HIGHLIGHT_COLOR = ft.Colors.BLUE_200

PAINT_COLORS = {
    "Красный": ft.Colors.RED_400,
    "Синий": ft.Colors.BLUE_400,
    "Зелёный": ft.Colors.GREEN_400,
    "Жёлтый": ft.Colors.YELLOW_400,
}

# ─── Правая часть: конфигурация ────────────
RIGHT_CONFIG = {
    "Золото":      {"count": 16, "arrows": [4, 8, 11, 14, 16]},
    "Нефть":       {"count": 16, "arrows": [4, 8, 12, 16]},
    "Nasdaq":      {"count": 15, "arrows": [3, 6, 9, 12, 15]},
    "Dow Jones":   {"count": 14, "arrows": [4, 8, 11, 14]},
    "Облигации":   {"count": 15, "arrows": [5, 10, 15]},
    "Акции стран": {"count": 12, "arrows": [2, 5, 8, 12]},
}

def main(page: ft.Page):
    page.title = "Хулиполия"
    page.bgcolor = ft.Colors.BLACK
    page.scroll = ft.ScrollMode.AUTO

    selected_color = {"value": ft.Colors.RED_400}

    # ─── Палитра цветов ─────────────────────
    palette = ft.Row(spacing=10)

    def select_color(color):
        selected_color["value"] = color
        for b in palette.controls:
            b.border = ft.border.all(
                3,
                ft.Colors.WHITE if b.bgcolor == color else ft.Colors.TRANSPARENT,
            )
        palette.update()

    for color in PAINT_COLORS.values():
        palette.controls.append(
            ft.Container(
                width=40,
                height=40,
                bgcolor=color,
                border_radius=6,
                border=ft.border.all(3, ft.Colors.TRANSPARENT),
                on_click=lambda e, c=color: select_color(c),
            )
        )

    # ─── Клик по клетке ──────────────────────
    def paint(e):
        cell = e.control
        if cell.bgcolor == selected_color["value"]:
            cell.bgcolor = (
                HIGHLIGHT_COLOR if getattr(cell, "highlight", False) else BASE_COLOR
            )
        else:
            cell.bgcolor = selected_color["value"]
        cell.update()

    # ─── Левая шкала ─────────────────────────
    def make_left_scale():
        cells = []
        for i in range(18):
            highlight = i in [0, 1, 2, 3, 16, 17]
            c = ft.Container(
                content=ft.Text(str(i), color="white"),
                width=32,
                height=32,
                alignment=ft.alignment.center,
                bgcolor=HIGHLIGHT_COLOR if highlight else BASE_COLOR,
                border_radius=4,
                on_click=paint,
            )
            c.highlight = highlight
            cells.append(c)
        return ft.Row(cells, spacing=4)

    # ─── Правая клетка ───────────────────────
    def make_right_cell(arrow=False):
        return ft.Container(
            content=ft.Text("↗" if arrow else "", color="white"),
            width=32,
            height=32,
            alignment=ft.alignment.center,
            bgcolor=BASE_COLOR,
            border_radius=4,
            on_click=paint,
        )

    # ─── Правая часть (2 строки) ─────────────
    def make_arrow_rows(count, arrows):
        rows = []
        for _ in range(2):
            cells = []
            for i in range(1, count + 1):
                is_arrow = i in arrows
                margin = ft.margin.only(right=10) if is_arrow else None
                cells.append(
                    ft.Container(
                        content=ft.Text("↗" if is_arrow else "", color="white"),
                        width=32,
                        height=32,
                        alignment=ft.alignment.center,
                        bgcolor=BASE_COLOR,
                        border_radius=4,
                        on_click=paint,
                        margin=margin,
                    )
                )
            rows.append(ft.Row(cells, spacing=4))
        return ft.Column(rows, spacing=4)

    # ─── Сборка строк ────────────────────────
    rows = []

    for title, cfg in RIGHT_CONFIG.items():
        left = make_left_scale()
        arrows = make_arrow_rows(cfg["count"], cfg["arrows"])

        rows.append(
            ft.Row(
                [
                    left,
                    ft.Text(title, width=160, color="white", weight="bold"),
                    arrows,
                ],
                alignment="start",
            )
        )

    page.add(
        ft.Column(
            [
                ft.Text("Выбери цвет:", color="white"),
                palette,
                ft.Divider(color="white"),
                *rows,
            ],
            spacing=14,
        )
    )

    select_color(selected_color["value"])

ft.app(target=main, view=ft.WEB_BROWSER)
