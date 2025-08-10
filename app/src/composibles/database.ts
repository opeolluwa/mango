import Database, {QueryResult} from "@tauri-apps/plugin-sql";



// const DB = await Database.load("sqlite:echo.db");

export class DbError extends Error {
    static NotFound = new DbError("Record not found");

    constructor(public readonly message: string) {
        super(message);
        this.name = "DbError";
    }

    static from(e: unknown) {
        return new DbError(`Database error: ${(e as Error).message}`);
    }
}

abstract class BaseModel {


    abstract save(): Promise<QueryResult>;

    protected async loadConn() {
        const conn = await Database.load("sqlite:echo.db");
        return conn;
    }

    // abstract findByIdentifier(identifier: string): Promise<any>;
    //
    // abstract update(identifier: string): Promise<void>;
    //
    // abstract delete(identifier: string): Promise<void>;
}

export class Playlist extends BaseModel {
    // private conn = Database;
    private readonly identifier: string;
    private readonly name: string;
    private readonly description: string;
    private readonly createdAt: string;
    private readonly updatedAt: string;

    constructor(name: string | null, description: string | null) {
        super();
        this.identifier ="";
        this.name = name || "";
        this.description = description || "";
        this.createdAt = new Date().toLocaleDateString();
        this.updatedAt = new Date().toLocaleDateString();
    }

    async save(): Promise<QueryResult> {
        const conn = await this.loadConn();
        try {
            return await conn.execute(
                "INSERT INTO playlist (identifier, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
                [
                    this.identifier,
                    this.name,
                    this.description,
                    this.createdAt,
                    this.updatedAt,
                ]
            );
        } catch (e) {
            throw DbError.from(e);
        }
    }

    // findByIdentifier(identifier: string): Promise<any> {
    //     throw new Error("Method not implemented.");
    // }
    //
    // update(identifier: string): Promise<void> {
    //     throw new Error("Method not implemented.");
    // }
    //
    // delete(identifier: string): Promise<void> {
    //     throw new Error("Method not implemented.");
    // }

    //
    // static async findById(conn: any, id: string): Promise<Playlist> {
    //     try {
    //         const row = await conn.selectOne<Playlist>(
    //             "SELECT * FROM playlist WHERE identifier = ?",
    //             [id]
    //         );
    //         if (!row) throw DbError.NotFound;
    //         return row;
    //     } catch (e) {
    //         throw DbError.from(e);
    //     }
    // }
    //
    // async update(conn: any): Promise<void> {
    //     try {
    //         await conn.execute(
    //             "UPDATE playlist SET name = ?, description = ? WHERE identifier = ?",
    //             [this.name, this.description, this.identifier]
    //         );
    //     } catch (e) {
    //         throw DbError.from(e);
    //     }
    // }
    //
    // static async delete(conn: any, id: string): Promise<void> {
    //     try {
    //         await conn.execute("DELETE FROM playlist WHERE identifier = ?", [id]);
    //     } catch (e) {
    //         throw DbError.from(e);
    //     }
    // }
}

export class History extends BaseModel {
    // private conn = DB;
    private readonly identifier: string;
    private readonly audioBookIdentifier: string;

    constructor(audioBookIdentifier: string) {
        super()
        this.identifier = "";
        this.audioBookIdentifier = audioBookIdentifier;
    }

    async save(): Promise<QueryResult> {
        try {
            const conn = await this.loadConn();
            return await conn.execute(
                "INSERT INTO history (identifier, audio_book_identifier) VALUES ($1, $2)",
                [this.identifier, this.audioBookIdentifier]
            );
        } catch (e) {
            throw DbError.from(e);
        }
    }

    // static async findById( id: string): Promise<History> {
    //     try {
    //         const row = await conn.selectOne<any>(
    //             "SELECT * FROM history WHERE identifier = ?",
    //             [id]
    //         );
    //         if (!row) throw DbError.NotFound;
    //
    //         // Convert DB row (with snake_case) to camelCase instance
    //         return new History(row.identifier, row.audio_book_identifier);
    //     } catch (e) {
    //         throw DbError.from(e);
    //     }
    // }

    // static async delete(conn: any, id: string): Promise<void> {
    //     try {
    //         await conn.execute("DELETE FROM history WHERE identifier = ?", [id]);
    //     } catch (e) {
    //         throw DbError.from(e);
    //     }
    // }
}
