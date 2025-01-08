using UnityEngine;

public class PlayerController : MonoBehaviour
{
    private BoardManager m_BoardManager;
    private Vector2Int m_CellPosition;

    public void Spawn(BoardManager boardManager, Vector2Int cellPosition)
    {
        m_BoardManager = boardManager;
        m_CellPosition = cellPosition;

        transform.position = m_BoardManager.CellToWorld(m_CellPosition);
    }


    void Start()
    {
        
    }

    void Update()
    {
        
    }
}
